"use client";

import { Button, Menu } from "@mantine/core";
import { MdKeyboardArrowDown } from "react-icons/md";

interface MenuItem {
  label: string;
  onClick: () => void;
}

interface Props {
  menuItems: MenuItem[];
  buttonLabel: string;
}

export const DropdownMenu = ({ menuItems, buttonLabel }: Props) => {
  return (
    <Menu
      transitionProps={{ transition: "pop-top-right" }}
      position="top-end"
      withinPortal
    >
      <Menu.Target>
        <Button
          variant="light"
          color="rgba(0, 0, 0, 1)"
          rightSection={<MdKeyboardArrowDown />}
          pr={12}
        >
          {buttonLabel}
        </Button>
      </Menu.Target>
      <Menu.Dropdown>
        {menuItems.map((item, index) => (
          <Menu.Item key={index} onClick={item.onClick}>
            {item.label}
          </Menu.Item>
        ))}
      </Menu.Dropdown>
    </Menu>
  );
};
