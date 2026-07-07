"use client";

import { useParams } from "next/navigation";
import { useCallback, useEffect, useState } from "react";
import Link from "next/link";
import { api, NoteSummary } from "@/lib/api";
import {
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar";
import NoteList from "@/components/NoteList";
import NoteViewer from "@/components/NoteViewer";
import { categoryLabel } from "@/lib/categories";

export default function NotePage() {
  const params = useParams();
  const id = params.id as string;

  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  useEffect(() => {
    api
      .listNotes()
      .then(setNotes)
      .catch(() => {})
      .finally(() => setLoading(false));
  }, []);

  const fetchNotes = useCallback(async (cat: string, q: string) => {
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
  }, []);

  const handleCategoryChange = useCallback(
    (cat: string) => {
      setCategory(cat);
      setSearchQuery("");
      fetchNotes(cat, "");
    },
    [fetchNotes]
  );

  const handleSearch = useCallback(
    (q: string) => {
      setSearchQuery(q);
      setCategory("");
      fetchNotes("", q);
    },
    [fetchNotes]
  );

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
          {/* Note list pane (desktop) */}
          <div className="hidden w-80 shrink-0 flex-col border-r border-border md:flex">
            <div className="flex items-center gap-2 border-b border-border px-4 py-3">
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
                activeId={id}
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

          {/* Detail pane */}
          <div className="flex min-w-0 flex-1 flex-col">
            {/* Mobile top bar */}
            <div className="flex items-center gap-2 border-b border-border px-4 py-2.5 md:hidden">
              <SidebarTrigger />
              <Link
                href="/"
                className="text-[13px] text-muted-foreground transition-colors hover:text-foreground"
              >
                ← 笔记列表
              </Link>
            </div>
            <NoteViewer id={id} />
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
