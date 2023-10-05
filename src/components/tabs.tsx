import { table } from "console";
import { Component, For } from "solid-js";

export class Tab {
    name!: string;
    compoment!: Element

    constructor(name: string, component: Component) {
        this.name = name;
        this.compoment = (component as unknown as Element);
    }
}

const ComponentTab: Component<{tab: Tab, my_index: number, currect_index: number}> = (props) => {
    return (
        <a class="tab tab-bordered flex-1" classList={{"tab-active": props.my_index === props.currect_index}}>{props.tab.name}</a>
    );
}

export const Tabs: Component<{tabs: Tab[], default_index: number}> = (props) => {
    return (
        <div class="flex-row w-full">
			<div class="flex">
                <For each={props.tabs}>
                    {(tab, index) => <ComponentTab tab={tab} my_index={index()} currect_index={props.default_index}/>}
                </For>
			</div>

            <div class="">
                {props.tabs[props.default_index].compoment}
            </div>
            
		</div>	
    );
}