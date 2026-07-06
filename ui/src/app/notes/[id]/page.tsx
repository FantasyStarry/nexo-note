'use client';

import { useParams } from 'next/navigation';
import { useCallback, useEffect, useState } from 'react';
import { api, NoteSummary } from '@/lib/api';
import { SidebarProvider, SidebarInset } from '@/components/ui/sidebar';
import AppSidebar from '@/components/AppSidebar';
import NoteList from '@/components/NoteList';
import NoteViewer from '@/components/NoteViewer';

export default function NotePage() {
  const params = useParams();
  const id = params.id as string;

  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState('');
  const [searchQuery, setSearchQuery] = useState('');

  useEffect(() => {
    api.listNotes().then(setNotes).catch(() => {}).finally(() => setLoading(false));
  }, []);

  const fetchNotes = useCallback(async (cat: string, q: string) => {
    setLoading(true);
    try {
      const result = await api.listNotes({ category: cat || undefined, q: q || undefined });
      setNotes(result);
    } catch {
      setNotes([]);
    }
    setLoading(false);
  }, []);

  const handleCategoryChange = useCallback((cat: string) => {
    setCategory(cat);
    setSearchQuery('');
    fetchNotes(cat, '');
  }, [fetchNotes]);

  const handleSearch = useCallback((q: string) => {
    setSearchQuery(q);
    setCategory('');
    fetchNotes('', q);
  }, [fetchNotes]);

  return (
    <SidebarProvider>
      <AppSidebar
        onCategoryChange={handleCategoryChange}
        onSearch={handleSearch}
        activeCategory={category}
      />
      <SidebarInset>
        <div className="flex h-screen">
          <div className="w-80 flex flex-col border-r">
            <div className="px-4 py-3 border-b">
              <h2 className="text-sm font-semibold">
                {category || (searchQuery ? `搜索: ${searchQuery}` : '全部笔记')}
              </h2>
            </div>
            {loading ? (
              <div className="flex-1 flex items-center justify-center">
                <div className="animate-spin w-5 h-5 border-2 border-primary border-t-transparent rounded-full" />
              </div>
            ) : (
              <NoteList notes={notes} activeId={id} />
            )}
          </div>
          <NoteViewer id={id} />
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
