import { Component, Index } from 'solid-js';
import { BsPeople } from 'solid-icons/bs';
import { VsTerminal } from 'solid-icons/vs';
import { FaRegularUser } from 'solid-icons/fa'

class Item {
	icon: any;
	text!: string;
	navigate_path!: string;

	constructor(text: string, path: string, icon: any,) {
		this.text = text;
		this.icon = icon;
		this.navigate_path = path;
	}
}

const Icon: Component<{ item: Item }> = (props) => {
	// TODO SolidIcon
	return (
		<li>
			<a class="tooltip tooltip-right rounded-none" data-tip={props.item.text}>
				{props.item.icon}
			</a>
		</li>
	);
};

const StantardItems = [
	new Item('Запросы в друзья', '/main/add', <BsPeople size={24} color="grey"/>),
	new Item('Выполнить комманду', '/main/run_command', <VsTerminal size={24} color="grey" />)
];

export const SideBar: Component = () => {
	return (
		<ul class="menu p-0">
			<Index each={StantardItems}>{(item, i) =>
				<div onclick={() => {
					console.error("43");
				}}>
					<Icon item={item()} />
				</div>
				}
			</Index>

			<hr style="border-color: grey;"/>

			<Icon item={new Item('Пользователь {dd}', 'ds', <FaRegularUser size={24} color="grey"/>)}/>
		</ul>
	);
};
