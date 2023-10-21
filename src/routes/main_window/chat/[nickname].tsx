import { SideBar } from '~/components/side_bar';
import { useParams } from 'solid-start';

export default function Index() {
	const params = useParams();

	return (
		<main class="flex">
			<SideBar />

			<p>{params.nickname}</p>
		</main>
	);
}
