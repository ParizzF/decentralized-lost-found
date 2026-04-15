// app.js
const contractId = "PASTE_CONTRACT_ID_DISINI";

// ⚠️ sementara mock dulu (biar bisa demo tanpa ribet wallet)
async function report() {
  const name = document.getElementById("name").value;
  const desc = document.getElementById("desc").value;

  alert("Simulasi kirim ke blockchain:\n" + name + " - " + desc);

  // nanti ini diganti invoke soroban
}

async function loadItems() {
  const list = document.getElementById("list");
  list.innerHTML = "";

  // dummy data (biar ada tampilan dulu)
  const items = [
    { name: "Dompet", status: "lost" },
    { name: "HP", status: "claimed" }
  ];

  items.forEach(i => {
    const li = document.createElement("li");
    li.innerText = `${i.name} - ${i.status}`;
    list.appendChild(li);
  });
}