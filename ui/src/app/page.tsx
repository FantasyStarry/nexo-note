'use client';

import { useCallback, useEffect, useState } from 'react';
import { api, NoteSummary } from '@/lib/api';
import { SidebarProvider, SidebarInset } from '@/components/ui/sidebar';
import AppSidebar from '@/components/AppSidebar';
import NoteList from '@/components/NoteList';

export default function HomePage() {
  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState('');
  const [searchQuery, setSearchQuery] = useState('');

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

  useEffect(() => {
    fetchNotes(category, searchQuery);
  }, [category, searchQuery, fetchNotes]);

  const handleCategoryChange = useCallback((cat: string) => {
    setCategory(cat);
    setSearchQuery('');
  }, []);

  const handleSearch = useCallback((q: string) => {
    setSearchQuery(q);
    setCategory('');
  }, []);

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
              <NoteList notes={notes} />
            )}
          </div>
          <div className="flex-1 flex items-center justify-center text-muted-foreground text-sm">
            选择一篇笔记查看详情
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
