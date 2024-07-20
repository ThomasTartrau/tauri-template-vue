import type { LucideIcon } from "lucide-vue-next";

export interface Submenu {
  route: string;
  label: string;
  active?: boolean;
}

export interface Menu {
  route: string;
  params?: string;
  label: string;
  active: boolean;
  icon: LucideIcon;
  submenus: Submenu[];
}

export interface Group {
  groupLabel: string;
  menus: Menu[];
}

export interface CollapseMenuButtonProps {
  icon: LucideIcon;
  label: string;
  active: boolean;
  submenus: Submenu[];
  // isOpen: boolean | undefined
}
