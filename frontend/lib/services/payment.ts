/**
 * Payment Gateway Integration Service
 *
 * Integrates with Razorpay for:
 * - Online payments (UPI, Cards, Net Banking, Wallets)
 * - Payment links
 * - Refunds
 * - Payment verification
 *
 * Documentation: https://razorpay.com/docs/
 */

export interface PaymentConfig {
  keyId: string;
  keySecret: string;
  webhookSecret?: string;
  environment: 'test' | 'live';
}

export interface PaymentOrder {
  orderId: string;
  amount: number; // in paise (₹1 = 100 paise)
  currency: 'INR';
  receipt: string;
  notes?: Record<string, string>;
}

export interface PaymentOptions {
  key: string;
  amount: number;
  currency: string;
  order_id: string;
  name: string;
  description: string;
  image?: string;
  prefill?: {
    name?: string;
    email?: string;
    contact?: string;
  };
  theme?: {
    color?: string;
  };
  handler: (response: PaymentResponse) => void;
  modal?: {
    ondismiss?: () => void;
  };
}

export interface PaymentResponse {
  razorpay_payment_id: string;
  razorpay_order_id: string;
  razorpay_signature: string;
}

export interface PaymentLink {
  id: string;
  short_url: string;
  amount: number;
  currency: string;
  description: string;
  customer: {
    name: string;
    email: string;
    contact: string;
  };
  expire_by?: number;
}

export interface RefundRequest {
  paymentId: string;
  amount?: number; // Partial refund (in paise), leave empty for full refund
  notes?: Record<string, string>;
}

export interface RefundResponse {
  id: string;
  payment_id: string;
  amount: number;
  currency: string;
  status: 'pending' | 'processed' | 'failed';
  created_at: number;
}

class PaymentService {
  private config: PaymentConfig;
  private razorpayInstance: unknown = null;

  constructor(config?: PaymentConfig) {
    this.config = config || {
      keyId: process.env.NEXT_PUBLIC_RAZORPAY_KEY_ID || '',
      keySecret: process.env.NEXT_PUBLIC_RAZORPAY_KEY_SECRET || '',
      webhookSecret: process.env.NEXT_PUBLIC_RAZORPAY_WEBHOOK_SECRET || '',
      environment: (process.env.NEXT_PUBLIC_RAZORPAY_ENV as 'test' | 'live') || 'test',
    };
  }

  /**
   * Load Razorpay script
   */
  private async loadRazorpayScript(): Promise<boolean> {
    return new Promise((resolve) => {
      if (typeof window === 'undefined') {
        resolve(false);
        return;
      }

      // Check if script is already loaded
      if ((window as { Razorpay?: unknown }).Razorpay) {
        resolve(true);
        return;
      }

      const script = document.createElement('script');
      script.src = 'https://checkout.razorpay.com/v1/checkout.js';
      script.async = true;
      script.onload = () => resolve(true);
      script.onerror = () => resolve(false);
      document.body.appendChild(script);
    });
  }

  /**
   * Create payment order
   */
  async createOrder(
    amount: number,
    receipt: string,
    notes?: Record<string, string>
  ): Promise<PaymentOrder> {
    try {
      const response = await fetch('/api/payment/create-order', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          amount: amount * 100, // Convert to paise
          currency: 'INR',
          receipt,
          notes,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to create payment order');
      }

      return await response.json();
    } catch (error) {
      console.error('Payment order creation error:', error);
      throw error;
    }
  }

  /**
   * Initialize payment checkout
   */
  async initiatePayment(
    orderId: string,
    amount: number,
    customerDetails: {
      name: string;
      email: string;
      contact: string;
    },
    description: string,
    onSuccess: (response: PaymentResponse) => void,
    onFailure?: (error: unknown) => void
  ): Promise<void> {
    try {
      const scriptLoaded = await this.loadRazorpayScript();

      if (!scriptLoaded) {
        throw new Error('Razorpay SDK failed to load');
      }

      const options: PaymentOptions = {
        key: this.config.keyId,
        amount: amount * 100, // Convert to paise
        currency: 'INR',
        order_id: orderId,
        name: 'Laboratory Information System',
        description,
        image: '/logo.png',
        prefill: {
          name: customerDetails.name,
          email: customerDetails.email,
          contact: customerDetails.contact,
        },
        theme: {
          color: '#3b82f6',
        },
        handler: onSuccess,
        modal: {
          ondismiss: () => {
            if (onFailure) {
              onFailure(new Error('Payment cancelled by user'));
            }
          },
        },
      };

      const razorpay = new (window as unknown as { Razorpay: new (options: PaymentOptions) => { open: () => void } }).Razorpay(options);
      razorpay.open();
    } catch (error) {
      console.error('Payment initiation error:', error);
      if (onFailure) {
        onFailure(error);
      }
      throw error;
    }
  }

  /**
   * Verify payment signature
   */
  async verifyPayment(
    orderId: string,
    paymentId: string,
    signature: string
  ): Promise<boolean> {
    try {
      const response = await fetch('/api/payment/verify', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          orderId,
          paymentId,
          signature,
        }),
      });

      if (!response.ok) {
        return false;
      }

      const data = await response.json();
      return data.verified === true;
    } catch (error) {
      console.error('Payment verification error:', error);
      return false;
    }
  }

  /**
   * Create payment link
   */
  async createPaymentLink(
    amount: number,
    description: string,
    customer: {
      name: string;
      email: string;
      contact: string;
    },
    expireBy?: Date
  ): Promise<PaymentLink> {
    try {
      const response = await fetch('/api/payment/create-link', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          amount: amount * 100, // Convert to paise
          currency: 'INR',
          description,
          customer,
          expire_by: expireBy ? Math.floor(expireBy.getTime() / 1000) : undefined,
          notify: {
            sms: true,
            email: true,
          },
          reminder_enable: true,
        }),
      });

      if (!response.ok) {
        throw new Error('Failed to create payment link');
      }

      return await response.json();
    } catch (error) {
      console.error('Payment link creation error:', error);
      throw error;
    }
  }

  /**
   * Get payment details
   */
  async getPaymentDetails(paymentId: string): Promise<unknown> {
    try {
      const response = await fetch(`/api/payment/${paymentId}`, {
        method: 'GET',
      });

      if (!response.ok) {
        throw new Error('Failed to fetch payment details');
      }

      return await response.json();
    } catch (error) {
      console.error('Payment details fetch error:', error);
      throw error;
    }
  }

  /**
   * Initiate refund
   */
  async initiateRefund(
    paymentId: string,
    amount?: number,
    notes?: Record<string, string>
  ): Promise<RefundResponse> {
    try {
      const response = await fetch('/api/payment/refund', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          paymentId,
          amount: amount ? amount * 100 : undefined, // Convert to paise if provided
          notes,
        }),
      });

      if (!response.ok) {
        throw new Error('Refund initiation failed');
      }

      return await response.json();
    } catch (error) {
      console.error('Refund error:', error);
      throw error;
    }
  }

  /**
   * Get refund status
   */
  async getRefundStatus(refundId: string): Promise<RefundResponse> {
    try {
      const response = await fetch(`/api/payment/refund/${refundId}`, {
        method: 'GET',
      });

      if (!response.ok) {
        throw new Error('Failed to fetch refund status');
      }

      return await response.json();
    } catch (error) {
      console.error('Refund status fetch error:', error);
      throw error;
    }
  }

  /**
   * Handle payment webhook
   */
  async handleWebhook(
    payload: string,
    signature: string
  ): Promise<{ verified: boolean; event: unknown }> {
    try {
      const response = await fetch('/api/payment/webhook', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Razorpay-Signature': signature,
        },
        body: payload,
      });

      if (!response.ok) {
        return { verified: false, event: null };
      }

      return await response.json();
    } catch (error) {
      console.error('Webhook handling error:', error);
      return { verified: false, event: null };
    }
  }

  /**
   * Check payment status
   */
  async checkPaymentStatus(
    orderId: string
  ): Promise<'pending' | 'authorized' | 'captured' | 'failed'> {
    try {
      const response = await fetch(`/api/payment/status/${orderId}`, {
        method: 'GET',
      });

      if (!response.ok) {
        return 'failed';
      }

      const data = await response.json();
      return data.status;
    } catch (error) {
      console.error('Payment status check error:', error);
      return 'failed';
    }
  }

  /**
   * Format amount for display
   */
  formatAmount(amountInPaise: number): string {
    return `₹${(amountInPaise / 100).toLocaleString('en-IN', {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })}`;
  }
}

// Singleton instance
export const paymentService = new PaymentService();

// Export class for custom configurations
export { PaymentService };
