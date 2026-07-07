"use client";

import { NoteSummary } from "@/lib/api";
import Link from "next/link";
import { CATEGORY_BADGE, CATEGORY_META, categoryLabel } from "@/lib/categories";
import { cn } from "@/lib/utils";

function timeAgo(dateStr: string): string {
  const ms = Date.now() - new Date(dateStr).getTime();
  const mins = Math.floor(ms / 60000);
  if (mins < 1) return "刚刚";
  if (mins < 60) return `${mins}分钟前`;
  const hours = Math.floor(mins / 60);
  if (hours < 24) return `${hours}小时前`;
  const days = Math.floor(hours / 24);
  if (days < 30) return `${days}天前`;
  return new Date(dateStr).toLocaleDateString("zh-CN");
}

export default function NoteList({
  notes,
  activeId,
  emptyMessage,
}: {
  notes: NoteSummary[];
  activeId?: string;
  emptyMessage?: string;
}) {
  if (notes.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center px-6 text-center text-[13px] text-muted-foreground">
        {emptyMessage || "暂无笔记"}
      </div>
    );
  }

  return (
    <div className="flex-1 overflow-y-auto">
      {notes.map((note) => {
        const active = activeId === note.id;
        const Icon = CATEGORY_META[note.category]?.icon;
        return (
          <Link
            key={note.id}
            href={`/notes/${note.id}`}
            aria-current={active ? "true" : undefined}
            className={cn(
              "block border-b border-l-2 border-border/70 border-l-transparent px-4 py-3 transition-colors duration-150",
              active
                ? "border-l-[#2383e2] bg-[rgba(55,53,47,0.07)]"
                : "hover:bg-[rgba(55,53,47,0.03)]"
            )}
          >
            <div className="mb-1.5 flex items-center gap-2">
              {Icon && <Icon className="size-3.5 shrink-0 text-muted-foreground" />}
              <span className="line-clamp-1 text-[14px] font-medium text-foreground/90">
                {note.title}
              </span>
            </div>
            <div className="flex flex-wrap items-center gap-x-2 gap-y-1 pl-5">
              <span
                className={cn(
                  "rounded px-1.5 py-0.5 text-[11px] font-medium",
                  CATEGORY_BADGE[note.category] || "bg-muted text-muted-foreground"
                )}
              >
                {categoryLabel(note.category)}
              </span>
              {note.tags.slice(0, 3).map((tag) => (
                <span key={tag} className="text-[11px] text-muted-foreground">
                  #{tag}
                </span>
              ))}
            </div>
            <p className="mt-1.5 pl-5 text-[11px] text-tertiary">
              {timeAgo(note.created_at)}
            </p>
          </Link>
        );
      })}
    </div>
  );
}
