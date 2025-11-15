import Cookies from 'js-cookie';

export interface User {
  id: string;
  email: string;
  name: string;
  organization_id: string;
  roles: string[];
  permissions: string[];
}

export interface AuthTokenPayload {
  sub: string;
  exp: number;
  organization_id: string;
  roles: string[];
  permissions: string[];
}

const TOKEN_KEY = 'auth_token';
const USER_KEY = 'auth_user';

export class AuthService {
  /**
   * Store authentication token
   */
  static setToken(token: string) {
    Cookies.set(TOKEN_KEY, token, {
      expires: 7, // 7 days
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'strict',
    });
  }

  /**
   * Get authentication token
   */
  static getToken(): string | undefined {
    return Cookies.get(TOKEN_KEY);
  }

  /**
   * Remove authentication token
   */
  static removeToken() {
    Cookies.remove(TOKEN_KEY);
  }

  /**
   * Store user data in localStorage
   */
  static setUser(user: User) {
    if (typeof window !== 'undefined') {
      localStorage.setItem(USER_KEY, JSON.stringify(user));
    }
  }

  /**
   * Get user data from localStorage
   */
  static getUser(): User | null {
    if (typeof window === 'undefined') return null;

    const userJson = localStorage.getItem(USER_KEY);
    if (!userJson) return null;

    try {
      return JSON.parse(userJson);
    } catch {
      return null;
    }
  }

  /**
   * Remove user data
   */
  static removeUser() {
    if (typeof window !== 'undefined') {
      localStorage.removeItem(USER_KEY);
    }
  }

  /**
   * Check if user is authenticated
   */
  static isAuthenticated(): boolean {
    const token = this.getToken();
    if (!token) return false;

    try {
      const payload = this.decodeToken(token);
      return payload.exp * 1000 > Date.now();
    } catch {
      return false;
    }
  }

  /**
   * Decode JWT token
   */
  static decodeToken(token: string): AuthTokenPayload {
    const base64Url = token.split('.')[1];
    const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
    const jsonPayload = decodeURIComponent(
      atob(base64)
        .split('')
        .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
        .join('')
    );

    return JSON.parse(jsonPayload);
  }

  /**
   * Check if user has specific role
   */
  static hasRole(role: string): boolean {
    const user = this.getUser();
    return user?.roles.includes(role) ?? false;
  }

  /**
   * Check if user has specific permission
   */
  static hasPermission(permission: string): boolean {
    const user = this.getUser();
    return user?.permissions.includes(permission) ?? false;
  }

  /**
   * Check if user has any of the specified permissions
   */
  static hasAnyPermission(permissions: string[]): boolean {
    const user = this.getUser();
    if (!user) return false;
    return permissions.some((permission) => user.permissions.includes(permission));
  }

  /**
   * Check if user has all of the specified permissions
   */
  static hasAllPermissions(permissions: string[]): boolean {
    const user = this.getUser();
    if (!user) return false;
    return permissions.every((permission) => user.permissions.includes(permission));
  }

  /**
   * Logout user
   */
  static logout() {
    this.removeToken();
    this.removeUser();

    if (typeof window !== 'undefined') {
      window.location.href = '/login';
    }
  }
}
