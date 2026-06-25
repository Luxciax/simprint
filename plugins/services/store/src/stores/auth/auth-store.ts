import { create } from 'zustand';

import type { User } from '../../types/store.types';

import type { AuthActions, AuthState } from './auth-store.types';

const LOCAL_USER: User = {
  uuid: 'local-user',
  id: 'local-user',
  nickname: 'Local User',
  email: 'local@simprint.local',
  phone: '',
  avatar: '',
  status: 'active',
  current_workspace_uuid: null,
  current_team_uuid: null,
};

/**
 * Auth Store - local no-login mode.
 * The frontend always sees an authenticated local user.
 */
export const useAuthStore = create<AuthState & AuthActions>((set, get) => ({
  user: LOCAL_USER,
  isAuthenticated: true,
  isInitializing: false,
  currentWorkspaceUuid: null,
  currentTeamUuid: null,

  setUser: (user: User) =>
    set({
      user,
      isAuthenticated: true,
      currentWorkspaceUuid: user.current_workspace_uuid || null,
      currentTeamUuid: user.current_team_uuid || null,
    }),

  clearUser: () =>
    set({
      user: LOCAL_USER,
      isAuthenticated: true,
      currentWorkspaceUuid: null,
      currentTeamUuid: null,
    }),

  setCurrentWorkspace: (workspaceUuid: string | null) =>
    set({
      currentWorkspaceUuid: workspaceUuid,
      user: {
        ...(get().user || LOCAL_USER),
        current_workspace_uuid: workspaceUuid,
      },
    }),

  setCurrentTeam: (teamUuid: string | null) =>
    set({
      currentTeamUuid: teamUuid,
      user: {
        ...(get().user || LOCAL_USER),
        current_team_uuid: teamUuid,
      },
    }),

  initAuth: async () => {
    set({
      user: LOCAL_USER,
      isAuthenticated: true,
      isInitializing: false,
      currentWorkspaceUuid: get().currentWorkspaceUuid,
      currentTeamUuid: get().currentTeamUuid,
    });
  },
}));
