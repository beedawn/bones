<script lang=ts>
	import { onMount } from "svelte";
	//imports data exported from page.server.js

    type Data ={
        bones: string
    }
	export let data:Data;

	//billData stores response from /b endpoint (list of bills user is permitted to access)
	let billData: ArrayLike<unknown> | { [s: string]: unknown; };
	//userData stores response from /u endpoint (list of users user is permitted to access?)
	let userData: ArrayLike<unknown> | { [s: string]: unknown; };

	//providerData stores response from /p endpoint (list of users user is permitted to access)
	let providerData: ArrayLike<unknown> | { [s: string]: unknown; };
	let statusData;
	let decodedJWT = 0;
	let username_response = null;

	onMount(async () => {
		//gets data from users table with JWT cookie
		fetch("http://localhost:8080/u", {
			method: "POST",
			credentials: "include",
		})
			.then((response) => response.json())
			.then((data) => {
				userData = data;
			})
			.catch((error) => {
				console.log(error);
				return [];
			});

		//gets data from users table with JWT cookie
		fetch("http://localhost:8080/p", {
			method: "POST",
			credentials: "include",
		})
			.then((response) => response.json())
			.then((data) => {
				providerData = data;
			})
			.catch((error) => {
				console.log(error);
				return [];
			});
		//gets data from bills table with JWT cookie
		fetch("http://localhost:8080/b", {
			method: "POST",
			credentials: "include",
		})
			.then((response) => response.json())
			.then((data) => {
				billData = data;
			})
			.catch((error) => {
				console.log(error);
				return [];
			});


            		//gets data from users table with JWT cookie
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

		let jwt = data.bones;
        let tokens: Array<string>;
        if(jwt){
		 tokens = jwt.split(".");}
        else{
            tokens=["",""];
        }
		//console.log(JSON.parse(atob(tokens[0])));
		//console.log(JSON.parse(atob(tokens[1])));
		//this nonsense gets the username to display on the page
		let tempJWT;
       try{
		tempJWT = JSON.parse(atob(tokens[1]));
        }
        catch{
            tempJWT="";

        }
		decodedJWT = tempJWT.role;
		username_response = tempJWT.sub;
	});
	//inputs for new bill
	let BillToAdd = {
		amount: "",
		date: "",
		img_path: "",
		duedate: "",
		providerid: "",
		billstatusid: "",
	};
	//logged in state to change from login prompt to dashboard view
	let loggedIn = false;
	//username variable from login prompt, used in requests to db
	let username:string = "";
	//password variable from login prompt, used in requests to db
	let password:string = "";
	let UserToAdd = {
		username: null,
		user_role_select: null,
		password: null,
		role: "",
		clear: function () {
			this.username = null;
			this.user_role_select = null;
			this.password = null;
			this.role = "";
		},
	};
enum viewMode{
	viewBill,
	addBill,
	Users,
	Provider
}
enum providerViewMode{
	viewProvider,
	addProvider,
	editProvider
}
	//handles a string to switch mode states, might be better if I just put a switch somewhere in a function to handle this
	let mode = viewMode.viewBill;
	//handles a string to switch provider mode states
	let providerMode = providerViewMode.viewProvider;
	//handles a string to handle user management mode states
	let userManagementMode = "";
	
	let ProviderToAdd={
		url:null,
		name:null,
		phone:null
	}

	let userbill_selection = [];
	let usermanagement_selection = [];
	let provider_selection = [];

	//runs when logout button is pressed
	async function logout() {
		//makes request to backend to remove stored cookie
		const res = await fetch("http://localhost:8080/logout", {
			method: "POST",
			credentials: "include",
		});
		//sets view loggedIn state to false to display login prompt
		loggedIn = false;
		//reloads page to update cookies
		location.reload();
	}
	//runs when login button is pressed
	async function login() {
		//makes request to backend to login
		//can we use SSL/TLS?
		const res = await fetch("http://localhost:8080/login", {
			method: "POST",
			credentials: "include",
			body: JSON.stringify({
				username,
				password,
			}),
		});

		const data = await res.json();

		UserToAdd.role = data.role;

		//sets loggedIn to true so UI shows user dashboard
		loggedIn = true;

		//reloads page to update cookies
		location.reload();
	}

	//potential function to add a new bill
	async function addBill() {
		//this endpoint does not exist yet
		const res = await fetch("http://localhost:8080/add_bill", {
			method: "POST",
			credentials: "include",
			body: JSON.stringify({
				amount: BillToAdd.amount,
				date: BillToAdd.date,
				img_path: BillToAdd.img_path,
				duedate: BillToAdd.duedate,
				providerid: BillToAdd.providerid,
				billstatusid: BillToAdd.billstatusid,
			}),
		});
		location.reload();
	}

	//potential function to add a new user
	async function addUser() {
		//this endpoint does not exist yet
		const res = await fetch("http://localhost:8080/add_user", {
			method: "POST",
			credentials: "include",
			body: JSON.stringify({
				username: UserToAdd.username,
				password: UserToAdd.password,
				role: UserToAdd.role,
			}),
		});
		//clears inputs
		console.log(UserToAdd.role);
		UserToAdd.clear(); //do i even need to clear it? since we reload?
		alert("User added!");
		location.reload();
	}
	async function billClick(billID) {
		if (userbill_selection.includes(billID)) {
			console.log("removing" + billID);
			userbill_selection = userbill_selection.filter(function (item) {
				return item !== billID;
			});
			return false;
		} else {
			userbill_selection.push(billID);
			userbill_selection = userbill_selection;
			console.log(userbill_selection);
			return true;
		}
	}

	async function userClick(userID) {
		if (usermanagement_selection.includes(userID)) {
			console.log("removing" + userID);
			usermanagement_selection = usermanagement_selection.filter(
				function (item) {
					return item !== userID;
				},
			);
			return false;
		} else {
			usermanagement_selection.push(userID);
			usermanagement_selection = usermanagement_selection;
			console.log(usermanagement_selection);
			return true;
		}
	}

	async function providerClick(providerID) {
		if (provider_selection.includes(providerID)) {
			console.log("removing" + providerID);
			provider_selection = provider_selection.filter(function (item) {
				return item !== providerID;
			});
			return false;
		} else {
			provider_selection.push(providerID);
			provider_selection = provider_selection;
			console.log(provider_selection);
			return true;
		}
	}

	//potential function to add a new provider
	async function addProvider() {
		//this endpoint does not exist yet
		const res = await fetch("http://localhost:8080/add_provider", {
			method: "POST",
			credentials: "include",
			body: JSON.stringify({
				url: ProviderToAdd.url,
				phone: ProviderToAdd.phone,
				name: ProviderToAdd.name,
			}),
		});
		location.reload();
	}
</script>

{#if !data.bones}
	<form>
		<label
			>Username
			<input data-testid="username_login"
				bind:value={username}
				on:keypress={(e) => {
					if (e.charCode == 32) e.preventDefault();
				}}
				placeholder="Username"
			/>
		</label>
		<label>
			Password
			<input data-testid="password_login"
				bind:value={password}
				type="password"
				on:keypress={(e) => {
					if (e.charCode == 32) e.preventDefault();
				}}
				placeholder="Password"
			/>
		</label>
		<button data-testid="button_login" type="submit" on:click={login}>Send it!</button>
	</form>
{/if}
{#if data.bones}
	<div>
		<div>Welcome {username_response}.</div>
		<!-- hi welcome to the bills app used to be here  -->
		<button
			on:click={() => {
				mode = viewMode.viewBill;
			}}
			aria-haspopup="true">View Bills</button
		>

		{#if decodedJWT == 1}
			<button
				on:click={() => {
					mode = viewMode.Users;
				}}
				aria-haspopup="true">User Management</button
			>{/if}
		{#if decodedJWT == 1 || decodedJWT == 2}<button
				on:click={() => {
					mode = viewMode.addBill;
				}}
				aria-haspopup="true"
			>
				Add Bill</button
			>
			<button
				on:click={() => {
					mode = viewMode.Provider;
					providerMode = providerViewMode.viewProvider;
				}}
				aria-haspopup="true"
			>
				Provider Management</button
			>
		{/if}
		<label> <button on:click={logout} name="logout"> Logout</button></label>
		{#if mode == viewMode.Users}
			<div>
				<button>Add User</button>
				<button>Edit User</button>
				<button>Delete User</button>
			</div>
			<div>
				<form>
					<label
						>New Username <input
							bind:value={UserToAdd.username}
							placeholder="New Username"
							name="newusername"
						/></label
					>
					<label>
						Set New User Password<input
							bind:value={UserToAdd.password}
							placeholder="New Password"
							name="newpassword"
						/></label
					>
					<select bind:value={UserToAdd.role} name="role" id="role">
						<option value="1">Admin</option>
						<option value="2" selected>Mod </option>
						<option value="3">User</option>
					</select>

					<button type="submit" on:click={addUser}>Add User</button>
				</form>

				<div>
					{#if userData}
						{#each Object.entries(userData) as [index, item]}
							<div
								on:click={() => userClick(item.id)}
								class={usermanagement_selection.includes(
									item.id,
								)
									? "selectedUser"
									: ""}
							>
								{item.id}
								<div class="inlineDiv">{item.username}</div>
								<div class="inlineDiv">
									{#if item.role == 1}Admin{/if}{#if item.role == 2}Mod{/if}{#if item.role == 3}User{/if}
								</div>
							</div>
						{/each}{/if}
				</div>
			</div>
		{/if}
		{#if mode == viewMode.Provider}
			<div>
				<button
					on:click={() => {
						providerMode = providerViewMode.addProvider;
					}}>Add Provider</button
				>
				<button
					on:click={() => {
						providerMode = providerViewMode.editProvider;
					}}>Edit Provider</button
				>
				{#if providerMode == providerViewMode.addProvider}
					<div>add provider</div>
					<form>
						<label
							>Provider Name<input
								bind:value={ProviderToAdd.name}
								placeholder="Provider Name"
								name="providername"
							/></label
						>
						<label
							>Provider URL<input
								bind:value={ProviderToAdd.url}
								placeholder="Provider URL"
								name="providerurl"
							/></label
						>
						<label
							>Provider Phone<input
								bind:value={ProviderToAdd.phone}
								placeholder="Provider Phone"
								name="providerphone"
							/></label
						>
						<button on:click={addProvider} type="submit"
							>Add Provider</button
						>
					</form>
				{/if}
				{#if providerMode == providerViewMode.editProvider}
					<div>edit provider</div>
				{/if}
				{#if providerData}
					<form>
						<select name="providers" multiple>
							{#each Object.entries(providerData) as [index, item]}
								<option>{item.name}</option>
								<!-- <div on:dragover|preventDefault={()=>console.log("hi")} on:click={()=>(providerClick(item.id))} class={provider_selection.includes(item.id) ? "selectedProvider":""}>
	<div class="inlineDiv">{item.id}</div>
	<div class="inlineDiv">{item.name}</div>
	<div class="inlineDiv">{item.url}</div>
	<div class="inlineDiv">{item.phone}</div>
</div> -->
							{/each}
						</select>
					</form>
				{/if}
			</div>
		{/if}

		{#if mode == viewMode.addBill}
			<div>
				<form>
					<!-- id | amount |   date    |  img_path   |  duedate  | providerid | billstatusid -->
					<!-- ----+--------+-----------+-------------+-----------+------------+-------------- -->
					<div class="bill_input">
						<label
							>Date Received<input
								bind:value={BillToAdd.date}
								placeholder="New Date Received"
								name="newdate"
							/></label
						>
					</div>
					<div class="bill_input">
						<label
							>Image Path<input
								bind:value={BillToAdd.img_path}
								placeholder="New Image Path"
								name="newimgpath"
							/></label
						>
					</div>
					<div class="bill_input">
						<label
							>Due Date<input
								bind:value={BillToAdd.duedate}
								placeholder="Due Date"
								name="newduedate"
							/></label
						>
					</div>
					<div class="bill_input">
						<label
							>Provider ID<input
								bind:value={BillToAdd.providerid}
								placeholder="Due Provider ID"
								name="newproviderid"
							/></label
						>
					</div>
					<div class="bill_input">
						<label
							>Provider ID<input
								bind:value={BillToAdd.billstatusid}
								placeholder="Due Bill Status ID"
								name="newbillstatusid"
							/></label
						>
					</div>
					<div class="bill_input">
						<label
							>New Bill Amount<input
								bind:value={BillToAdd.amount}
								placeholder="Amount Due"
								name="amountdue"
							/></label
						>
					</div>
					<div class="bill_button">
						<button type="submit" on:click={addBill}
							>Add Bill</button
						>
					</div>
				</form>
			</div>
		{/if}

		{#if mode == viewMode.viewBill}
			<div>
		
				<table class="styled-table">
					<thead>
						<tr>
							<th>Link</th>
							<th>ID</th>
							<th>Bill Received</th>
							<th>Bill Status</th>
							<th> Amount Due</th>
							<th>image path</th>
							<th>Due Date</th>
							<th>Provider</th>
						</tr>
					</thead>
					{#if billData}
						{#each Object.entries(billData) as [index, item]}
							<tbody>
								<tr
									on:click={() => billClick(item.id)}
									class={userbill_selection.includes(item.id)
										? "selectedBill"
										: ""}
								>
									<td><a href="/bill/{item.id}">Link</a></td>
									<td>{item.id}</td>
									<td>{item.date}</td>
									<td>{item.bill_status}</td>
									<td>{item.amount}</td>
									<td>{item.img_path}</td>
									<td>{item.duedate}</td>
									<td>{item.provider_name}</td>
								</tr>
							</tbody>

							<br />
						{/each}
					{/if}
				</table>
			</div>
		{/if}
	</div>
{/if}

<style>
	@import "../../static/style.css";
</style>
