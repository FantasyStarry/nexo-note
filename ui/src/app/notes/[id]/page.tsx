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
import NoteTable from "@/components/NoteTable";
import NoteViewer from "@/components/NoteViewer";
import { categoryLabel } from "@/lib/categories";
import { cn } from "@/lib/utils";
import { LayoutList, Table } from "lucide-react";

export default function NotePage() {
  const params = useParams();
  const id = params.id as string;

  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [category, setCategory] = useState("");
  const [tag, setTag] = useState("");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedId, setSelectedId] = useState<string | null>(id);
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

  // Restore the active filter from the URL so opening a note keeps the
  // category / tag context (Notion-like: the sidebar selection persists).
  useEffect(() => {
    const params = new URLSearchParams(
      typeof window !== "undefined" ? window.location.search : ""
    );
    const cat = params.get("cat") ?? "";
    const tg = params.get("tag") ?? "";
    const q = params.get("q") ?? "";
    setCategory(cat);
    setTag(tg);
    setSearchQuery(q);
    fetchNotes(cat, tg, q);
  }, [fetchNotes]);

  const handleCategoryChange = useCallback(
    (cat: string) => {
      setCategory(cat);
      setTag("");
      setSearchQuery("");
      fetchNotes(cat, "", "");
    },
    [fetchNotes]
  );

  const handleTagSelect = useCallback(
    (tg: string) => {
      setTag(tg);
      setCategory("");
      setSearchQuery("");
      fetchNotes("", tg, "");
    },
    [fetchNotes]
  );

  const handleSearch = useCallback(
    (q: string) => {
      setSearchQuery(q);
      setCategory("");
      setTag("");
      fetchNotes("", "", q);
    },
    [fetchNotes]
  );

  const listTitle = category
    ? categoryLabel(category)
    : tag
      ? `标签: #${tag}`
      : searchQuery
        ? `搜索: ${searchQuery}`
        : "全部笔记";

  const emptyMessage = category
    ? `「${categoryLabel(category)}」分类下暂无笔记`
    : tag
      ? `标签「${tag}」下暂无笔记`
      : searchQuery
        ? `未找到与「${searchQuery}」相关的笔记`
        : undefined;

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
          {/* Note list pane (desktop) */}
          <div className="hidden w-80 shrink-0 flex-col border-r border-border md:flex">
            <div className="flex items-center gap-2 border-b border-border px-4 py-3">
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
                activeId={selectedId ?? id}
                category={category}
                search={searchQuery}
                tag={tag}
                onSelect={setSelectedId}
                emptyMessage={emptyMessage}
              />
            ) : (
              <NoteTable
                notes={notes}
                activeId={selectedId ?? id}
                category={category}
                search={searchQuery}
                tag={tag}
                onSelect={setSelectedId}
                emptyMessage={emptyMessage}
              />
            )}
          </div>

          {/* Detail pane */}
          <div className="flex min-w-0 flex-1 flex-col">
            {/* Mobile top bar */}
            <div className="flex items-center gap-2 border-b border-border px-4 py-2.5 md:hidden">
              <SidebarTrigger />
              <Link
                href={`/${[category && `cat=${encodeURIComponent(category)}`, tag && `tag=${encodeURIComponent(tag)}`, searchQuery && `q=${encodeURIComponent(searchQuery)}`].filter(Boolean).join("&")}`}
                className="text-[13px] text-muted-foreground transition-colors hover:text-foreground"
              >
                ← 笔记列表
              </Link>
            </div>
            <NoteViewer id={selectedId ?? id} />
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
