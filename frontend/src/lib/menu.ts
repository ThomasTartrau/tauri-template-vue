import { LucideIcon } from "lucide-vue-next"

export interface Submenu {
    href: string
    label: string
    active?: boolean
  }
  
  export interface Menu {
    href: string
    label: string
    active: boolean
    icon: LucideIcon
    submenus: Submenu[]
  }
  
  export interface Group {
    groupLabel: string
    menus: Menu[]
  }
  
  export interface CollapseMenuButtonProps {
    icon: LucideIcon
    label: string
    active: boolean
    submenus: Submenu[]
    // isOpen: boolean | undefined
  }