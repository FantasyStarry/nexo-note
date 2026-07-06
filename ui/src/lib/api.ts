// API types matching Rust backend

export interface NoteSummary {
  id: string;
  title: string;
  category: string;
  tags: string[];
  status: string;
  created_at: string;
  file_path?: string;
}

export interface NoteDetail {
  id: string;
  title: string;
  category: string;
  tags: string[];
  status: string;
  content: string;
  file_path: string;
  parent_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface ThreadNote {
  id: string;
  title: string;
  category: string;
  tags: string[];
  status: string;
  parent_id: string | null;
  created_at: string;
}

export interface ThreadData {
  notes: ThreadNote[];
  total: number;
}

export interface TagCount {
  tag: string;
  count: number;
}

export interface StatsData {
  total_notes: number;
  this_month: number;
  total_tags: number;
  top_tags: TagCount[];
}

export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: { code: string; message: string; suggestion?: string };
}

// API client

const API_BASE = '/api';

async function fetchJson<T>(url: string): Promise<T> {
  const res = await fetch(`${API_BASE}${url}`);
  if (!res.ok) throw new Error(`API error: ${res.status}`);
  const json: ApiResponse<T> = await res.json();
  if (!json.success || json.data === undefined) {
    throw new Error(json.error?.message || 'Unknown API error');
  }
  return json.data;
}

export const api = {
  listNotes: (params?: { category?: string; tag?: string; q?: string; limit?: number }) => {
    const sp = new URLSearchParams();
    if (params?.category) sp.set('category', params.category);
    if (params?.tag) sp.set('tag', params.tag);
    if (params?.q) sp.set('q', params.q);
    if (params?.limit) sp.set('limit', String(params.limit));
    const qs = sp.toString();
    return fetchJson<NoteSummary[]>(`/notes${qs ? `?${qs}` : ''}`);
  },

  getNote: (id: string) =>
    fetchJson<NoteDetail>(`/notes/${encodeURIComponent(id)}`),

  getThread: (id: string) =>
    fetchJson<ThreadData>(`/thread/${encodeURIComponent(id)}`),

  listTags: () =>
    fetchJson<TagCount[]>('/tags'),

  getStats: () =>
    fetchJson<StatsData>('/stats'),
};
