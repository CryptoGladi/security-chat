import { Component } from 'solid-js';
import './side_bar.css';

class Item {
    icon: any;
    text!: string;

    constructor(text: string, icon: any) {
        this.text = text;
        this.icon = icon;
    }
}

const Icon: Component<{ item: Item }> = (props) => { // TODO SolidIcon
	return (
		<div class="sidebar-icon group">
			{props.item.icon}

			<span class="sidebar-tooltip">{props.item.text}</span>
		</div>
	);
};

export const SideBar: Component = () => {
	return (
		<div class="fixed left-0 top-0 m-0 flex h-screen w-16 flex-col text-white">
			<Icon item={new Item("sa", "ds")}></Icon>
            <Icon item={new Item("sa", "d")}></Icon>
		</div>
	);
};
