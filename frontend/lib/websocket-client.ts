import { io, Socket } from 'socket.io-client';
import Cookies from 'js-cookie';

export type EventHandler = (data: unknown) => void;

export interface DomainEvent {
  event_id: string;
  event_type: string;
  aggregate_id: string;
  aggregate_type: string;
  payload: unknown;
  metadata: {
    organization_id: string;
    user_id?: string;
    timestamp: string;
    correlation_id?: string;
    causation_id?: string;
  };
}

class WebSocketClient {
  private socket: Socket | null = null;
  private eventHandlers: Map<string, Set<EventHandler>> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 3;
  private enabled = false;
  private connectionFailed = false;

  constructor() {
    // Check if WebSocket is enabled via environment variable
    const wsEnabled = process.env.NEXT_PUBLIC_ENABLE_WEBSOCKET === 'true';
    if (wsEnabled) {
      this.enabled = true;
      this.connect();
    } else {
      console.info('[WebSocket] Disabled. Set NEXT_PUBLIC_ENABLE_WEBSOCKET=true to enable real-time features.');
    }
  }

  private connect() {
    if (!this.enabled || this.connectionFailed) return;

    const token = Cookies.get('auth_token');
    const wsUrl = process.env.NEXT_PUBLIC_WS_URL || 'http://localhost:9000';

    this.socket = io(wsUrl, {
      auth: {
        token,
      },
      reconnection: true,
      reconnectionDelay: 2000,
      reconnectionDelayMax: 10000,
      reconnectionAttempts: this.maxReconnectAttempts,
      timeout: 5000,
    });

    this.setupListeners();
  }

  private setupListeners() {
    if (!this.socket) return;

    this.socket.on('connect', () => {
      console.info('[WebSocket] Connected successfully');
      this.reconnectAttempts = 0;
      this.connectionFailed = false;
    });

    this.socket.on('disconnect', (reason) => {
      console.info('[WebSocket] Disconnected:', reason);
    });

    this.socket.on('connect_error', (error) => {
      if (this.reconnectAttempts === 0) {
        console.warn('[WebSocket] Connection failed. Real-time features disabled.', error.message);
      }
    });

    this.socket.on('reconnect_attempt', (attempt) => {
      this.reconnectAttempts = attempt;
      if (attempt === 1) {
        console.info('[WebSocket] Attempting to reconnect...');
      }
    });

    this.socket.on('reconnect_failed', () => {
      this.connectionFailed = true;
      console.warn('[WebSocket] All reconnection attempts failed. Real-time features will be disabled.');
      // Disconnect to stop further attempts
      this.disconnect();
    });

    // Listen for domain events
    this.socket.on('domain_event', (event: DomainEvent) => {
      this.handleDomainEvent(event);
    });
  }

  private handleDomainEvent(event: DomainEvent) {
    // Trigger all handlers registered for this event type
    const handlers = this.eventHandlers.get(event.event_type);
    if (handlers) {
      handlers.forEach((handler) => handler(event));
    }

    // Trigger wildcard handlers
    const wildcardHandlers = this.eventHandlers.get('*');
    if (wildcardHandlers) {
      wildcardHandlers.forEach((handler) => handler(event));
    }
  }

  // Subscribe to specific event types
  on(eventType: string, handler: EventHandler): () => void {
    if (!this.eventHandlers.has(eventType)) {
      this.eventHandlers.set(eventType, new Set());
    }
    this.eventHandlers.get(eventType)!.add(handler);

    // Return unsubscribe function
    return () => {
      this.eventHandlers.get(eventType)?.delete(handler);
    };
  }

  // Join a specific room (e.g., organization, patient, sample)
  joinRoom(room: string) {
    if (!this.socket) return;
    this.socket.emit('join_room', room);
  }

  // Leave a room
  leaveRoom(room: string) {
    if (!this.socket) return;
    this.socket.emit('leave_room', room);
  }

  // Disconnect
  disconnect() {
    if (this.socket) {
      this.socket.disconnect();
      this.socket = null;
    }
    this.eventHandlers.clear();
  }

  // Check connection status
  isConnected(): boolean {
    return this.socket?.connected ?? false;
  }
}

// Singleton instance
let wsClient: WebSocketClient | null = null;

export function getWebSocketClient(): WebSocketClient {
  if (typeof window === 'undefined') {
    // Return a mock client for SSR
    return {
      on: () => () => {},
      joinRoom: () => {},
      leaveRoom: () => {},
      disconnect: () => {},
      isConnected: () => false,
    } as unknown as WebSocketClient;
  }

  if (!wsClient) {
    wsClient = new WebSocketClient();
  }

  return wsClient;
}

export default WebSocketClient;
