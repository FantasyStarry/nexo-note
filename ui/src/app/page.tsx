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
import NoteTable from "@/components/NoteTable";
import NoteViewer from "@/components/NoteViewer";
import { FileText, LayoutList, Table } from "lucide-react";
import { categoryLabel } from "@/lib/categories";
import { cn } from "@/lib/utils";

export default function HomePage() {
  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState("");
  const [tag, setTag] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<"list" | "table">("list");

  useEffect(() => {
    const saved = localStorage.getItem("nexo-view-mode");
    if (saved === "list" || saved === "table") {
      setViewMode(saved as "list" | "table");
    }
  }, []);

  const handleViewMode = useCallback((mode: "list" | "table") => {
    setViewMode(mode);
    localStorage.setItem("nexo-view-mode", mode);
  }, []);

  const emptyMessage = category
    ? `「${categoryLabel(category)}」分类下暂无笔记`
    : tag
      ? `标签「${tag}」下暂无笔记`
      : searchQuery
        ? `未找到与「${searchQuery}」相关的笔记`
        : undefined;

  const fetchNotes = useCallback(
    async (cat: string, tg: string, q: string) => {
      setLoading(true);
      try {
        const result = await api.listNotes({
          category: cat || undefined,
          tag: tg || undefined,
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
    fetchNotes(category, tag, searchQuery);
  }, [category, tag, searchQuery, fetchNotes]);

  const handleCategoryChange = useCallback((cat: string) => {
    setCategory(cat);
    setTag("");
    setSearchQuery("");
  }, []);

  const handleTagSelect = useCallback((tg: string) => {
    setTag(tg);
    setCategory("");
    setSearchQuery("");
  }, []);

  const handleSearch = useCallback((q: string) => {
    setSearchQuery(q);
    setCategory("");
    setTag("");
  }, []);

  const listTitle = category
    ? categoryLabel(category)
    : tag
      ? `标签: #${tag}`
      : searchQuery
        ? `搜索: ${searchQuery}`
        : "全部笔记";

  return (
    <SidebarProvider>
      <AppSidebar
        onCategoryChange={handleCategoryChange}
        onTagSelect={handleTagSelect}
        onSearch={handleSearch}
        activeCategory={category}
        activeTag={tag}
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
              <div className="ml-auto flex items-center gap-2">
                <div className="flex items-center rounded-md border border-border/60 p-0.5">
                  <button
                    type="button"
                    onClick={() => handleViewMode("list")}
                    aria-label="列表视图"
                    className={cn(
                      "rounded p-1 transition-colors",
                      viewMode === "list"
                        ? "bg-background text-foreground shadow-sm"
                        : "text-muted-foreground hover:text-foreground"
                    )}
                  >
                    <LayoutList className="size-3.5" />
                  </button>
                  <button
                    type="button"
                    onClick={() => handleViewMode("table")}
                    aria-label="表格视图"
                    className={cn(
                      "rounded p-1 transition-colors",
                      viewMode === "table"
                        ? "bg-background text-foreground shadow-sm"
                        : "text-muted-foreground hover:text-foreground"
                    )}
                  >
                    <Table className="size-3.5" />
                  </button>
                </div>
                {!loading && (
                  <span className="text-[12px] tabular-nums text-tertiary">
                    {notes.length}
                  </span>
                )}
              </div>
            </div>
            {loading && notes.length === 0 ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="size-5 animate-spin rounded-full border-2 border-border border-t-foreground/40" />
              </div>
            ) : viewMode === "list" ? (
              <NoteList
                notes={notes}
                category={category}
                search={searchQuery}
                tag={tag}
                onSelect={setSelectedId}
                emptyMessage={emptyMessage}
              />
            ) : (
              <NoteTable
                notes={notes}
                category={category}
                search={searchQuery}
                tag={tag}
                onSelect={setSelectedId}
                emptyMessage={emptyMessage}
              />
            )}
          </div>

          {/* Detail pane (desktop) */}
          <div className="hidden flex-1 flex-col bg-background md:flex">
            {selectedId ? (
              <NoteViewer id={selectedId} />
            ) : (
              <div className="flex flex-1 flex-col items-center justify-center text-center">
                <FileText className="mb-3 size-7 text-muted-foreground/40" />
                <p className="text-[13px] text-muted-foreground/60">选择一篇笔记查看详情</p>
              </div>
            )}
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
