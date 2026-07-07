"use client";

import { useState, useEffect, useCallback } from "react";
import { api, TagCount, NoteSummary } from "@/lib/api";
import { Search } from "lucide-react";
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarInput,
  SidebarSeparator,
  SidebarMenuBadge,
} from "@/components/ui/sidebar";
import { CATEGORY_META } from "@/lib/categories";

export default function AppSidebar({
  onCategoryChange,
  onSearch,
  activeCategory,
}: {
  onCategoryChange: (cat: string) => void;
  onSearch: (q: string) => void;
  activeCategory: string;
}) {
  const [tags, setTags] = useState<TagCount[]>([]);
  const [query, setQuery] = useState("");
  const [catCounts, setCatCounts] = useState<Record<string, number>>({});
  const [total, setTotal] = useState(0);

  useEffect(() => {
    api.listTags().then(setTags).catch(() => {});
    api
      .listNotes()
      .then((notes: NoteSummary[]) => {
        const counts: Record<string, number> = {};
        for (const n of notes) {
          counts[n.category] = (counts[n.category] || 0) + 1;
        }
        setCatCounts(counts);
        setTotal(notes.length);
      })
      .catch(() => {});
  }, []);

  // Keyboard shortcut: Ctrl+K / Cmd+K to focus search.
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === "k") {
        e.preventDefault();
        const searchInput = document.querySelector(
          '[data-sidebar="true"] input'
        ) as HTMLInputElement;
        searchInput?.focus();
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, []);

  const handleSearch = useCallback(
    (e: React.FormEvent) => {
      e.preventDefault();
      onSearch(query);
    },
    [query, onSearch]
  );

  return (
    <Sidebar>
      <SidebarContent>
        {/* Workspace mark */}
        <div className="px-3 pb-1 pt-4">
          <div className="flex items-center gap-2 px-1">
            <div className="flex size-6 items-center justify-center rounded-md bg-foreground/90 text-[11px] font-semibold text-background">
              N
            </div>
            <span className="text-[15px] font-semibold tracking-tight text-foreground">
              nexo
            </span>
          </div>
        </div>

        {/* Search */}
        <div className="px-3 py-2">
          <form onSubmit={handleSearch} className="relative">
            <Search className="pointer-events-none absolute left-2.5 top-1/2 size-4 -translate-y-1/2 text-muted-foreground" />
            <SidebarInput
              placeholder="搜索... (Ctrl+K)"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Escape") {
                  setQuery("");
                  onSearch("");
                }
              }}
              className="h-8 border-transparent bg-black/[0.035] pl-8 text-[13px] hover:bg-black/[0.05] focus-visible:border-border focus-visible:bg-background dark:bg-white/[0.04] dark:hover:bg-white/[0.06]"
            />
          </form>
        </div>

        <SidebarSeparator />

        {/* Categories */}
        <SidebarGroup className="py-1">
          <SidebarGroupContent>
            <SidebarMenu>
              {Object.entries(CATEGORY_META).map(([key, meta]) => {
                const Icon = meta.icon;
                return (
                  <SidebarMenuItem key={key}>
                    <SidebarMenuButton
                      isActive={activeCategory === key}
                      onClick={() => onCategoryChange(key)}
                      tooltip={meta.label}
                      className="gap-2.5 px-2.5 text-[14px] text-muted-foreground"
                    >
                      <Icon className="size-[18px] shrink-0 text-current/80" />
                      <span className="font-medium">{meta.label}</span>
                    </SidebarMenuButton>
                    <SidebarMenuBadge>
                      <span className="rounded bg-black/[0.05] px-1.5 py-0.5 text-[11px] tabular-nums text-muted-foreground dark:bg-white/[0.07]">
                        {key === "" ? total : (catCounts[key] ?? 0)}
                      </span>
                    </SidebarMenuBadge>
                  </SidebarMenuItem>
                );
              })}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>

        <SidebarSeparator />

        {/* Tags */}
        {tags.length > 0 && (
          <SidebarGroup>
            <SidebarGroupLabel className="px-3 text-[11px] font-medium uppercase tracking-wider text-tertiary">
              标签
            </SidebarGroupLabel>
            <SidebarGroupContent>
              <SidebarMenu>
                {tags.map((t) => (
                  <SidebarMenuItem key={t.tag}>
                    <SidebarMenuButton
                      onClick={() => {
                        onCategoryChange("");
                        onSearch(t.tag);
                      }}
                      size="sm"
                      className="gap-1.5 px-2.5 text-[13px] text-muted-foreground"
                    >
                      <span className="text-muted-foreground/70">#</span>
                      <span className="flex-1 truncate">{t.tag}</span>
                      <SidebarMenuBadge className="ml-auto">
                        <span className="rounded bg-black/[0.05] px-1.5 py-0.5 text-[11px] tabular-nums text-muted-foreground dark:bg-white/[0.07]">
                          {t.count}
                        </span>
                      </SidebarMenuBadge>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </SidebarGroup>
        )}
      </SidebarContent>
    </Sidebar>
  );
}
