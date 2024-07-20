import { useStorage } from "@vueuse/core";

export function useAdminPanel() {
  const isOpen = useStorage("panel-is-open", () => true);

  const setOpen = () => {
    isOpen.value = !isOpen.value;
  };

  return {
    isOpen,
    setOpen,
  };
}
