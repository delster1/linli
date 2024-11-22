interface PostData {
	id: number;
	content: string;
	likes: number;
	date: string;
	author: string;
}

function TitleBar({ username }: {username: string} ) {
	
	return ( <header className="flex-1 inline-flex justify-center justify-items-center w-full m-4">
		<button className="text-center bg-teal-800 border-teal-950 border-solid border-4 rounded-l-lg p-4">{username}</button>
		<div className="text-6xl bg-teal-800 text-center p-4 border-teal-950 border-solid border-y-4">Linli ❤️</div>
		<button className="text-center bg-teal-800 border-teal-950 border-solid border-4 rounded-r-lg p-4">Jump</button>
	</ header>)
	
}

function Post (post: PostData) {
	return ( 
		<div className="rounded-lg  border-4 border-sky-700 flex-1 inline-flex justify-start justify-items-center bg-violet-800 w-9/12 m-10">

			<div className="flex-initial inline-flex flex-col justify-center justify-items-center p-8">
				<div className="">{post.author}</div>
				<div className="">{post.date}</div>
			</div>
			
			<div className="inline-flex flex-col justify-center">
				<div className="border-l-4 border-sky-700 text-wrap break-all max-w-l max-h-fit p-2">{post.content}</div>
			</div>
		</div>

	);
}

export default async function Home() {
const data = await fetch('https://127.0.0.1/getpost?server=server1&index=0')
const posts = await data.json()

const feed = posts.posts.map(p => <li className="inline-flex justify-center justify-items-center w-full max-w-l" key={p.id} ><Post {...p} /></li>)
  return (
<div>
	<TitleBar username="james" />
	<ul>{feed}</ul>
</div>
	);
}
