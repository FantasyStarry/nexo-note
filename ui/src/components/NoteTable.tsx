"use client";

import { useRouter } from "next/navigation";
import { NoteSummary } from "@/lib/api";
import { CATEGORY_META, categoryLabel } from "@/lib/categories";
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

export default function NoteTable({
  notes,
  activeId,
  emptyMessage,
  category,
  search,
  tag,
  onSelect,
}: {
  notes: NoteSummary[];
  activeId?: string;
  emptyMessage?: string;
  category?: string;
  search?: string;
  tag?: string;
  onSelect?: (id: string) => void;
}) {
  const router = useRouter();

  if (notes.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center px-6 text-center text-[13px] text-muted-foreground">
        {emptyMessage || "暂无笔记"}
      </div>
    );
  }

  const qs = [
    category && `cat=${encodeURIComponent(category)}`,
    tag && `tag=${encodeURIComponent(tag)}`,
    search && `q=${encodeURIComponent(search)}`,
  ]
    .filter(Boolean)
    .join("&");

  const handleRowClick = (noteId: string) => {
    if (
      onSelect &&
      typeof window !== "undefined" &&
      !window.matchMedia("(max-width: 767px)").matches
    ) {
      onSelect(noteId);
    } else {
      router.push(`/notes/${noteId}${qs ? `?${qs}` : ""}`);
    }
  };

  return (
    <div className="flex-1 overflow-y-auto">
      <table className="w-full border-collapse text-[13px]">
        <thead>
          <tr className="border-b border-border text-left text-[12px] text-muted-foreground">
            <th className="px-4 py-2.5 font-medium">标题</th>
            <th className="px-4 py-2.5 font-medium">分类</th>
            <th className="px-4 py-2.5 font-medium">标签</th>
            <th className="px-4 py-2.5 font-medium">时间</th>
          </tr>
        </thead>
        <tbody>
          {notes.map((note) => {
            const active = activeId === note.id;
            const Icon = CATEGORY_META[note.category]?.icon;
            return (
              <tr
                key={note.id}
                role="button"
                onClick={() => handleRowClick(note.id)}
                className={cn(
                  "cursor-pointer border-b border-border/60 transition-colors duration-150",
                  active
                    ? "bg-[rgba(13,125,114,0.07)]"
                    : "hover:bg-[rgba(42,39,34,0.03)]"
                )}
              >
                <td className="px-4 py-2.5">
                  <div className="flex items-center gap-2">
                    {Icon && (
                      <Icon className="size-3.5 shrink-0 text-muted-foreground" />
                    )}
                    <span className="line-clamp-1 font-medium text-foreground/90">
                      {note.title}
                    </span>
                  </div>
                </td>
                <td className="px-4 py-2.5 text-muted-foreground">
                  {categoryLabel(note.category)}
                </td>
                <td className="px-4 py-2.5">
                  <div className="flex flex-wrap gap-1">
                    {note.tags.slice(0, 3).map((t) => (
                      <span
                        key={t}
                        className="text-[11px] text-muted-foreground"
                      >
                        {t}
                      </span>
                    ))}
                  </div>
                </td>
                <td className="px-4 py-2.5 text-muted-foreground">
                  {timeAgo(note.created_at)}
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
