"use client";

import { useCallback, useEffect, useState } from "react";
import { api, NoteSummary } from "@/lib/api";
import {
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar";
import NoteList from "@/components/NoteList";
import { FileText } from "lucide-react";
import { categoryLabel } from "@/lib/categories";

export default function HomePage() {
  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  const fetchNotes = useCallback(
    async (cat: string, q: string) => {
      setLoading(true);
      try {
        const result = await api.listNotes({
          category: cat || undefined,
          q: q || undefined,
        });
        setNotes(result);
      } catch {
        setNotes([]);
      }
      setLoading(false);
    },
    []
  );

  useEffect(() => {
    fetchNotes(category, searchQuery);
  }, [category, searchQuery, fetchNotes]);

  const handleCategoryChange = useCallback((cat: string) => {
    setCategory(cat);
    setSearchQuery("");
  }, []);

  const handleSearch = useCallback((q: string) => {
    setSearchQuery(q);
    setCategory("");
  }, []);

  const listTitle = category
    ? categoryLabel(category)
    : searchQuery
      ? `搜索: ${searchQuery}`
      : "全部笔记";

  return (
    <SidebarProvider>
      <AppSidebar
        onCategoryChange={handleCategoryChange}
        onSearch={handleSearch}
        activeCategory={category}
      />
      <SidebarInset>
        <div className="flex h-screen">
          {/* Note list pane */}
          <div className="flex w-full shrink-0 flex-col border-r border-border md:w-80">
            <div className="flex items-center gap-2 border-b border-border px-4 py-3">
              <SidebarTrigger className="md:hidden" />
              <h2 className="text-[15px] font-semibold tracking-tight text-foreground">
                {listTitle}
              </h2>
              {!loading && (
                <span className="ml-auto text-[12px] tabular-nums text-tertiary">
                  {notes.length}
                </span>
              )}
            </div>
            {loading ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="size-5 animate-spin rounded-full border-2 border-border border-t-foreground/40" />
              </div>
            ) : (
              <NoteList
                notes={notes}
                emptyMessage={
                  category
                    ? `「${categoryLabel(category)}」分类下暂无笔记`
                    : searchQuery
                      ? `未找到与「${searchQuery}」相关的笔记`
                      : undefined
                }
              />
            )}
          </div>

          {/* Empty detail placeholder (desktop) */}
          <div className="hidden flex-1 flex-col items-center justify-center bg-background text-center md:flex">
            <FileText className="mb-3 size-10 text-border" />
            <p className="text-[14px] text-muted-foreground">选择一篇笔记查看详情</p>
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
