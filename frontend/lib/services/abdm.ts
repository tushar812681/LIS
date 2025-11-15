/**
 * ABDM (Ayushman Bharat Digital Mission) Integration Service
 *
 * Integrates with India's national health stack for:
 * - ABHA (Ayushman Bharat Health Account) number generation
 * - Health records sharing
 * - Patient verification
 * - Consent management
 *
 * Documentation: https://abdm.gov.in/
 */

export interface ABDMConfig {
  clientId: string;
  clientSecret: string;
  baseUrl: string;
  environment: 'sandbox' | 'production';
}

export interface ABHAAddress {
  healthId: string;
  healthIdNumber: string;
  name: string;
  gender: 'M' | 'F' | 'O';
  dateOfBirth: string;
  mobile: string;
  email?: string;
  address: string;
  districtCode: string;
  stateCode: string;
  pincode: string;
}

export interface ABHACard {
  healthIdNumber: string;
  healthId: string;
  name: string;
  photo: string;
  dateOfBirth: string;
  gender: string;
  mobile: string;
  address: string;
}

export interface ConsentRequest {
  patientId: string;
  purpose: 'CAREMGT' | 'BTG' | 'PUBHLTH' | 'HPAYMT' | 'DSRCH' | 'PATRQT';
  hiTypes: string[];
  dateRange: {
    from: string;
    to: string;
  };
  dataEraseAt: string;
}

export interface HealthRecord {
  recordId: string;
  patientId: string;
  recordType: string;
  content: unknown;
  createdAt: string;
  provider: string;
}

class ABDMService {
  private config: ABDMConfig;
  private accessToken: string | null = null;
  private tokenExpiry: number | null = null;

  constructor(config?: ABDMConfig) {
    this.config = config || {
      clientId: process.env.NEXT_PUBLIC_ABDM_CLIENT_ID || '',
      clientSecret: process.env.NEXT_PUBLIC_ABDM_CLIENT_SECRET || '',
      baseUrl: process.env.NEXT_PUBLIC_ABDM_BASE_URL || 'https://dev.abdm.gov.in',
      environment: (process.env.NEXT_PUBLIC_ABDM_ENV as 'sandbox' | 'production') || 'sandbox',
    };
  }

  /**
   * Get or refresh access token
   */
  private async getAccessToken(): Promise<string> {
    // Check if token is still valid
    if (this.accessToken && this.tokenExpiry && Date.now() < this.tokenExpiry) {
      return this.accessToken;
    }

    try {
      const response = await fetch(`${this.config.baseUrl}/gateway/v0.5/sessions`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          clientId: this.config.clientId,
          clientSecret: this.config.clientSecret,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to obtain ABDM access token');
      }

      const data = await response.json();
      this.accessToken = data.accessToken;
      this.tokenExpiry = Date.now() + (data.expiresIn * 1000);

      return this.accessToken!;
    } catch (error) {
      console.error('ABDM authentication error:', error);
      throw new Error('ABDM authentication failed');
    }
  }

  /**
   * Generate OTP for ABHA number creation
   */
  async generateABHACreationOTP(mobile: string): Promise<{ txnId: string }> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/api/v2/registration/aadhaar/generateOtp`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ mobile }),
      });

      if (!response.ok) {
        throw new Error('Failed to generate OTP');
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM OTP generation error:', error);
      throw error;
    }
  }

  /**
   * Verify OTP and create ABHA number
   */
  async verifyOTPAndCreateABHA(
    txnId: string,
    otp: string
  ): Promise<ABHAAddress> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/api/v2/registration/aadhaar/verifyOTP`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ txnId, otp }),
      });

      if (!response.ok) {
        throw new Error('OTP verification failed');
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM OTP verification error:', error);
      throw error;
    }
  }

  /**
   * Search for existing ABHA number
   */
  async searchABHAByHealthId(healthId: string): Promise<ABHAAddress | null> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/api/v1/search/searchByHealthId`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ healthId }),
      });

      if (!response.ok) {
        return null;
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM search error:', error);
      return null;
    }
  }

  /**
   * Get ABHA card (PDF/PNG)
   */
  async getABHACard(healthIdNumber: string): Promise<ABHACard> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/api/v1/account/getCard`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({ healthIdNumber }),
      });

      if (!response.ok) {
        throw new Error('Failed to fetch ABHA card');
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM card fetch error:', error);
      throw error;
    }
  }

  /**
   * Request consent for health information access
   */
  async requestConsent(consentRequest: ConsentRequest): Promise<{ consentId: string }> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/gateway/v0.5/consent-requests/init`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({
          requestId: crypto.randomUUID(),
          timestamp: new Date().toISOString(),
          consent: {
            purpose: {
              code: consentRequest.purpose,
            },
            patient: {
              id: consentRequest.patientId,
            },
            hiTypes: consentRequest.hiTypes,
            permission: {
              dateRange: consentRequest.dateRange,
              dataEraseAt: consentRequest.dataEraseAt,
            },
          },
        }),
      });

      if (!response.ok) {
        throw new Error('Consent request failed');
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM consent request error:', error);
      throw error;
    }
  }

  /**
   * Get consent status
   */
  async getConsentStatus(consentId: string): Promise<{ status: string }> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/gateway/v0.5/consents/${consentId}`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error('Failed to fetch consent status');
      }

      return await response.json();
    } catch (error) {
      console.error('ABDM consent status error:', error);
      throw error;
    }
  }

  /**
   * Share health records
   */
  async shareHealthRecords(
    consentId: string,
    records: HealthRecord[]
  ): Promise<{ success: boolean }> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/gateway/v0.5/health-information/transfer`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({
          requestId: crypto.randomUUID(),
          timestamp: new Date().toISOString(),
          consentId,
          records,
        }),
      });

      if (!response.ok) {
        throw new Error('Health record sharing failed');
      }

      return { success: true };
    } catch (error) {
      console.error('ABDM record sharing error:', error);
      throw error;
    }
  }

  /**
   * Link patient to ABHA
   */
  async linkPatientToABHA(
    patientId: string,
    healthId: string
  ): Promise<{ linked: boolean }> {
    try {
      const token = await this.getAccessToken();
      const response = await fetch(`${this.config.baseUrl}/api/v1/patients/link`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`,
        },
        body: JSON.stringify({
          patientId,
          healthId,
        }),
      });

      if (!response.ok) {
        throw new Error('Patient linking failed');
      }

      return { linked: true };
    } catch (error) {
      console.error('ABDM patient linking error:', error);
      throw error;
    }
  }

  /**
   * Verify ABDM service configuration
   */
  async verifyConfiguration(): Promise<boolean> {
    try {
      // Check if configuration is present
      if (!this.config.clientId || !this.config.clientSecret) {
        return false;
      }
      // Try to get access token to verify credentials
      await this.getAccessToken();
      return true;
    } catch {
      return false;
    }
  }
}

// Singleton instance
export const abdmService = new ABDMService();

// Export class for custom configurations
export { ABDMService };
