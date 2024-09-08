<script lang="ts">
	import { onMount } from 'svelte';
	//imports data exported from page.server.js

	type Data = {
		bones: string;
		params: string;
	};
	export let data: Data;

	//billData stores response from /b endpoint (list of bills user is permitted to access)
	let billData: ArrayLike<unknown> | { [s: string]: unknown; };
	let providerData;
    let statusData;

	onMount(async () => {
		//gets data from bills table with JWT cookie
		//need to set up back end to only fetch 1 bill
		fetch('http://localhost:8080/b/' + data.params, {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((data) => {
				billData = data;
                
			})
			.catch((error) => {
				console.log(error);
				return [];
			});

		//gets data from providers table with JWT cookie
		fetch('http://localhost:8080/p', {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((data) => {
				providerData = data;
			})
			.catch((error) => {
				console.log(error);
				return [];
			});

            		//gets data from billstatus table with JWT cookie
		fetch('http://localhost:8080/s', {
			method: 'POST',
			credentials: 'include'
		})
			.then((response) => response.json())
			.then((data) => {
				statusData = data;
			})
			.catch((error) => {
				console.log(error);
				return [];
			});
	});
enum billViewMode {
view,
edit
}
	let mode = billViewMode.view;

</script>

<a href="/">Back</a>

{#if billData}
	{#each Object.entries(billData) as [index, item]}
		<button
			on:click={() => {
				mode = billViewMode.edit;
			}}>Edit</button
		>
		{#if item.id == data.params && mode == billViewMode.view}
			<tbody>
				<tr>
					{item.id}
					<td>{item.date}</td>
					<td>{item.bill_status}</td>
					<td>{item.amount}</td>
					<td>{item.img_path}</td>
					<td>{item.duedate}</td>
					<td>{item.provider_name}</td>
				</tr>
			</tbody>

			<br />
		{/if}

		{#if item.id == data.params && mode == billViewMode.edit}
			<button
				on:click={() => {
					mode = billViewMode.view;
				}}>Close</button
			>
			<tbody>
				<tr>
					{item.id}
					<td><input value={item.date} /></td>
					<td>
                    
                    	<select name="providers" id="cars">
							{#each Object.entries(statusData) as [index, status]}
                            {#if status.id == item.billstatusid}
                            <option value={status.id} selected
									>{status.status}</option
								>{/if}
                                {#if status.id != item.billstatusid}
                                <option value={status.id} 
                                        >{status.status}</option
                                    >{/if}
                                
                                {/each}</select
						>
                    
                    </td>
					<td><input value={item.amount} /></td>
					<td><input value={item.img_path} /></td>
					<td> <input value={item.duedate} /></td>

					<td>
						<select name="providers" id="provider">
							{#each Object.entries(providerData) as [index, provider]}
                            {#if item.providerid == provider.id}
                            <option value={provider.id} selected
                            >{provider.name}</option
                        >{/if}
                        {#if item.providerid != provider.id}
                            <option value={provider.id}
									>{provider.name}</option
								>
                                {/if}{/each}</select
						></td
					>
				</tr>
				<button>Submit</button>
			</tbody>

		
		{/if}
	{/each}
{/if}
