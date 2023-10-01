import { SideBar } from '~/components/side_bar';
import { Repl, ReplTab } from 'solid-repl/dist/types';

export default function Index() {
	return (
		<main class="flex">
			<SideBar />

			<Repl // BUG
      baseUrl="https://solid-playground.netlify.app"
      height={500}
      withHeader
      isInteractive
    >
      <ReplTab name="main">
        {`
          import { render } from 'solid-js/web';
          import { App } from './app.tsx';
          
          render(App, document.getElementById('app'));
        `}
      </ReplTab>
      <ReplTab name="app">
        {'export const App = () => <h1>Hello world</h1>'}
      </ReplTab>
    </Repl>

		</main>
	);
}
