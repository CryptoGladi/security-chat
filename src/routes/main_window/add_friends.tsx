import { SideBar } from '~/components/side_bar';
import { Tabs, Tab } from '~/components/tabs';
import { AllFriends } from '~/components/add_friends/all_friends';
import { RequestsFriends } from '~/components/add_friends/requests_friends';

const ConstTabs: Tab[] = [new Tab('Ваши друзья', AllFriends), new Tab('Запросы', AllFriends), new Tab('Заявки', RequestsFriends)];

export default function Index() {

	return (
		<main class="flex">
			<SideBar />
			<Tabs tabs={ConstTabs} default_index={0}/>	
		</main>
	);
}
