/**
 * WorkOS AuthKit Configuration for Phenotype
 * 
 * Auto-generated from: https://significant-vessel-93-staging.authkit.app/.well-known/oauth-authorization-server
 * 
 * @module @phenotype/authkit-config
 */

import { WorkOS } from '@workos-inc/node';

// ============================================================================
// OAuth Discovery Configuration
// ============================================================================

export const AUTHKIT_CONFIG = {
  // OAuth 2.0 Authorization Server Metadata
  issuer: process.env.WORKOS_ISSUER || 'https://significant-vessel-93-staging.authkit.app',
  
  authorizationEndpoint: process.env.WORKOS_AUTHORIZATION_ENDPOINT || 
    'https://significant-vessel-93-staging.authkit.app/oauth2/authorize',
  
  tokenEndpoint: process.env.WORKOS_TOKEN_ENDPOINT || 
    'https://significant-vessel-93-staging.authkit.app/oauth2/token',
  
  introspectionEndpoint: process.env.WORKOS_INTROSPECTION_ENDPOINT || 
    'https://significant-vessel-93-staging.authkit.app/oauth2/introspection',
  
  registrationEndpoint: process.env.WORKOS_REGISTRATION_ENDPOINT || 
    'https://significant-vessel-93-staging.authkit.app/oauth2/register',
  
  // PKCE Configuration
  codeChallengeMethods: ['S256'] as const,
  
  // Grant Types
  grantTypes: ['authorization_code', 'refresh_token'] as const,
  
  // Response Types
  responseTypes: ['code'] as const,
  
  // Response Modes
  responseModes: ['query'] as const,
  
  // Scopes
  scopes: ['openid', 'profile', 'email', 'offline_access'] as const,
  
  // Token Endpoint Auth Methods
  tokenEndpointAuthMethods: [
    'none',
    'client_secret_post',
    'client_secret_basic'
  ] as const,
} as const;

// ============================================================================
// Client Configuration
// ============================================================================

export const WORKOS_CLIENT_CONFIG = {
  apiKey: process.env.WORKOS_API_KEY,
  clientId: process.env.WORKOS_CLIENT_ID,
  redirectUri: process.env.NEXT_PUBLIC_WORKOS_REDIRECT_URI || 'http://localhost:3000/callback',
  cookiePassword: process.env.WORKOS_COOKIE_PASSWORD,
} as const;

// ============================================================================
// WorkOS SDK Instance
// ============================================================================

export function createWorkOSClient(): WorkOS {
  const apiKey = WORKOS_CLIENT_CONFIG.apiKey;
  
  if (!apiKey) {
    throw new Error(
      'WORKOS_API_KEY is not defined. ' +
      'Please set it in your environment variables.'
    );
  }
  
  return new WorkOS(apiKey, {
    clientId: WORKOS_CLIENT_CONFIG.clientId,
  });
}

// Singleton instance (for server-side usage)
let workosInstance: WorkOS | null = null;

export function getWorkOS(): WorkOS {
  if (!workosInstance) {
    workosInstance = createWorkOSClient();
  }
  return workosInstance;
}

// ============================================================================
// Authorization URL Builder
// ============================================================================

export interface AuthorizationUrlOptions {
  state?: string;
  codeChallenge?: string;
  codeChallengeMethod?: 'S256';
  scopes?: string[];
  redirectUri?: string;
}

export function buildAuthorizationUrl(options: AuthorizationUrlOptions = {}): string {
  const {
    state,
    codeChallenge,
    codeChallengeMethod = 'S256',
    scopes = ['openid', 'profile', 'email'],
    redirectUri = WORKOS_CLIENT_CONFIG.redirectUri,
  } = options;
  
  const params = new URLSearchParams({
    client_id: WORKOS_CLIENT_CONFIG.clientId || '',
    response_type: 'code',
    redirect_uri: redirectUri || '',
    scope: scopes.join(' '),
  });
  
  if (state) {
    params.set('state', state);
  }
  
  if (codeChallenge) {
    params.set('code_challenge', codeChallenge);
    params.set('code_challenge_method', codeChallengeMethod);
  }
  
  return `${AUTHKIT_CONFIG.authorizationEndpoint}?${params.toString()}`;
}

// ============================================================================
// Type Definitions
// ============================================================================

export interface AuthKitUser {
  id: string;
  email: string;
  firstName?: string;
  lastName?: string;
  picture?: string;
  emailVerified: boolean;
  profile?: Record<string, unknown>;
}

export interface AuthKitSession {
  accessToken: string;
  refreshToken?: string;
  user: AuthKitUser;
  expiresAt: Date;
}

export interface TokenResponse {
  access_token: string;
  refresh_token?: string;
  id_token?: string;
  token_type: 'Bearer';
  expires_in: number;
  scope: string;
}

// ============================================================================
// PKCE Utilities
// ============================================================================

export function generateCodeVerifier(): string {
  const array = new Uint8Array(32);
  crypto.getRandomValues(array);
  return btoa(String.fromCharCode(...array))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

export async function generateCodeChallenge(verifier: string): Promise<string> {
  const encoder = new TextEncoder();
  const data = encoder.encode(verifier);
  const digest = await crypto.subtle.digest('SHA-256', data);
  return btoa(String.fromCharCode(...new Uint8Array(digest)))
    .replace(/\+/g, '-')
    .replace(/\//g, '_')
    .replace(/=/g, '');
}

// ============================================================================
// Default Export
// ============================================================================

export default {
  config: AUTHKIT_CONFIG,
  clientConfig: WORKOS_CLIENT_CONFIG,
  buildAuthorizationUrl,
  generateCodeVerifier,
  generateCodeChallenge,
  getWorkOS,
  createWorkOSClient,
};
