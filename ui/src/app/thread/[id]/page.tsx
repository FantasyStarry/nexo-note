"use client";

import { useParams, useRouter } from "next/navigation";
import { useCallback, useEffect, useState } from "react";
import Link from "next/link";
import { api, NoteSummary, ThreadData } from "@/lib/api";
import {
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar";
import NoteList from "@/components/NoteList";
import ThreadGraph from "@/components/ThreadGraph";
import { categoryLabel } from "@/lib/categories";

function buildQuery(cat: string, tag: string, q: string): string {
  return [
    cat && `cat=${encodeURIComponent(cat)}`,
    tag && `tag=${encodeURIComponent(tag)}`,
    q && `q=${encodeURIComponent(q)}`,
  ]
    .filter(Boolean)
    .join("&");
}

export default function ThreadPage() {
  const params = useParams();
  const router = useRouter();
  const id = params.id as string;

  const [notes, setNotes] = useState<NoteSummary[]>([]);
  const [loadingList, setLoadingList] = useState(true);
  const [category, setCategory] = useState("");
  const [tag, setTag] = useState("");
  const [searchQuery, setSearchQuery] = useState("");

  const [thread, setThread] = useState<ThreadData | null>(null);
  const [loadingThread, setLoadingThread] = useState(true);

  const fetchNotes = useCallback(async (cat: string, tg: string, q: string) => {
    setLoadingList(true);
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
    setLoadingList(false);
  }, []);

  // Restore the active filter from the URL so the left list keeps context.
  useEffect(() => {
    const sp = new URLSearchParams(
      typeof window !== "undefined" ? window.location.search : ""
    );
    const cat = sp.get("cat") ?? "";
    const tg = sp.get("tag") ?? "";
    const q = sp.get("q") ?? "";
    setCategory(cat);
    setTag(tg);
    setSearchQuery(q);
    fetchNotes(cat, tg, q);
  }, [fetchNotes]);

  useEffect(() => {
    setLoadingThread(true);
    api
      .getThread(id)
      .then(setThread)
      .catch(() => {})
      .finally(() => setLoadingThread(false));
  }, [id]);

  const currentNote = thread?.notes.find((n) => n.id === id);

  // Category / tag / search from the sidebar navigate back to the main list
  // (no dead-ends inside the thread view).
  const goToList = useCallback(
    (cat: string, tg: string, q: string) => {
      const qs = buildQuery(cat, tg, q);
      router.push(qs ? `/?${qs}` : "/");
    },
    [router]
  );

  const handleCategoryChange = useCallback(
    (cat: string) => {
      setCategory(cat);
      setTag("");
      setSearchQuery("");
      goToList(cat, "", "");
    },
    [goToList]
  );

  const handleTagSelect = useCallback(
    (tg: string) => {
      setTag(tg);
      setCategory("");
      setSearchQuery("");
      goToList("", tg, "");
    },
    [goToList]
  );

  const handleSearch = useCallback(
    (q: string) => {
      setSearchQuery(q);
      setCategory("");
      setTag("");
      goToList("", "", q);
    },
    [goToList]
  );

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
          {/* Note list pane (desktop) */}
          <div className="hidden w-80 shrink-0 flex-col border-r border-border md:flex">
            <div className="flex items-center gap-2 border-b border-border px-4 py-3">
              <h2 className="text-[15px] font-semibold tracking-tight text-foreground">
                {listTitle}
              </h2>
              {!loadingList && (
                <span className="ml-auto text-[12px] tabular-nums text-tertiary">
                  {notes.length}
                </span>
              )}
            </div>
            {loadingList && notes.length === 0 ? (
              <div className="flex flex-1 items-center justify-center">
                <div className="size-5 animate-spin rounded-full border-2 border-border border-t-foreground/40" />
              </div>
            ) : (
              <NoteList
                notes={notes}
                activeId={id}
                category={category}
                search={searchQuery}
                tag={tag}
                emptyMessage={
                  category
                    ? `「${categoryLabel(category)}」分类下暂无笔记`
                    : tag
                      ? `标签「${tag}」下暂无笔记`
                      : searchQuery
                        ? `未找到与「${searchQuery}」相关的笔记`
                        : undefined
                }
              />
            )}
          </div>

          {/* Thread pane */}
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

            {/* Header */}
            <div className="flex items-center justify-between border-b border-border px-6 py-4">
              <div>
                <p className="text-[11px] font-medium uppercase tracking-wider text-tertiary">
                  笔记链
                </p>
                <h1 className="text-[18px] font-semibold tracking-tight">
                  {currentNote?.title || "..."}
                </h1>
              </div>
              <Link
                href={`/notes/${id}`}
                className="text-[13px] text-muted-foreground transition-colors hover:text-foreground"
              >
                ← 返回笔记
              </Link>
            </div>

            {/* Graph */}
            <div className="flex flex-1 items-center justify-center overflow-y-auto">
              {loadingThread ? (
                <div className="size-5 animate-spin rounded-full border-2 border-border border-t-foreground/40" />
              ) : thread && thread.notes.length > 1 ? (
                <ThreadGraph notes={thread.notes} currentId={id} />
              ) : (
                <p className="text-[13px] text-muted-foreground">暂无笔记链</p>
              )}
            </div>
          </div>
        </div>
      </SidebarInset>
    </SidebarProvider>
  );
}
