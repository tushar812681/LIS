import { create } from 'zustand';
import { AuthService, User } from './auth';

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;
  setUser: (user: User | null) => void;
  logout: () => void;
  checkAuth: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: AuthService.getUser(),
  isAuthenticated: AuthService.isAuthenticated(),
  setUser: (user) => {
    if (user) {
      AuthService.setUser(user);
    } else {
      AuthService.removeUser();
    }
    set({ user, isAuthenticated: !!user });
  },
  logout: () => {
    AuthService.logout();
    set({ user: null, isAuthenticated: false });
  },
  checkAuth: () => {
    const isAuthenticated = AuthService.isAuthenticated();
    const user = isAuthenticated ? AuthService.getUser() : null;
    set({ user, isAuthenticated });
  },
}));

interface NotificationState {
  notifications: Array<{
    id: string;
    type: 'success' | 'error' | 'warning' | 'info';
    title: string;
    message: string;
    timestamp: Date;
  }>;
  addNotification: (notification: Omit<NotificationState['notifications'][0], 'id' | 'timestamp'>) => void;
  removeNotification: (id: string) => void;
  clearNotifications: () => void;
}

export const useNotificationStore = create<NotificationState>((set) => ({
  notifications: [],
  addNotification: (notification) => {
    const id = Math.random().toString(36).substring(7);
    set((state) => ({
      notifications: [
        ...state.notifications,
        { ...notification, id, timestamp: new Date() },
      ],
    }));

    // Auto-remove after 5 seconds
    setTimeout(() => {
      set((state) => ({
        notifications: state.notifications.filter((n) => n.id !== id),
      }));
    }, 5000);
  },
  removeNotification: (id) => {
    set((state) => ({
      notifications: state.notifications.filter((n) => n.id !== id),
    }));
  },
  clearNotifications: () => {
    set({ notifications: [] });
  },
}));

interface SidebarState {
  isCollapsed: boolean;
  toggle: () => void;
  collapse: () => void;
  expand: () => void;
}

export const useSidebarStore = create<SidebarState>((set) => ({
  isCollapsed: false,
  toggle: () => set((state) => ({ isCollapsed: !state.isCollapsed })),
  collapse: () => set({ isCollapsed: true }),
  expand: () => set({ isCollapsed: false }),
}));
