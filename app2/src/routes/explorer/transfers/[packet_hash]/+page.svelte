<script lang="ts">
import { page } from "$app/state"
import { onMount } from "svelte"
import { Effect, Fiber, Option } from "effect"
import { transferByPacketHashQuery } from "$lib/queries/transfer-by-hash.svelte"
import Card from "$lib/components/ui/Card.svelte"
import Sections from "$lib/components/ui/Sections.svelte"
import Label from "$lib/components/ui/Label.svelte"
import ErrorComponent from "$lib/components/model/ErrorComponent.svelte"
import ChainComponent from "$lib/components/model/ChainComponent.svelte"
import TokenComponent from "$lib/components/model/TokenComponent.svelte"
import TransactionHashComponent from "$lib/components/model/TransactionHashComponent.svelte"
import BlockHashComponent from "$lib/components/model/BlockHashComponent.svelte"
import AddressComponent from "$lib/components/model/AddressComponent.svelte"
import DateTimeComponent from "$lib/components/ui/DateTimeComponent.svelte"
import { chains } from "$lib/stores/chains.svelte"
import { settingsStore } from "$lib/stores/settings.svelte"
import { getChain } from "@unionlabs/sdk/schema"
import PacketComponent from "$lib/components/model/PacketComponent.svelte"
import { packetDetailsQuery } from "$lib/queries/packet-details.svelte"
import { packetDetails } from "$lib/stores/packets.svelte"

// Store for the transfer details
import { transferDetails } from "$lib/stores/transfer-details.svelte"
import SharpRightArrowIcon from "$lib/components/icons/SharpRightArrowIcon.svelte"
import PacketTracesComponent from "$lib/components/model/PacketTracesComponent.svelte"

// State for packet details visibility
let showPacketDetails = false

let fiber: Fiber.Fiber<any, any>
const packetHash = page.params.packet_hash

onMount(() => {
  fiber = Effect.runFork(transferByPacketHashQuery(packetHash))
  packetDetails.runEffect(packetDetailsQuery(packetHash))

  return async () => {
    await Effect.runPromise(Fiber.interrupt(fiber))
    transferDetails.data = Option.none()
    transferDetails.error = Option.none()

    // Clean up packet details
    packetDetails.interruptFiber()
  }
})
</script>

<Sections>
  <Card class="overflow-auto" divided>
    <div class="p-4">Transfer Details</div>
    <div>
    {#if Option.isSome(transferDetails.error)}
      <ErrorComponent error={transferDetails.error.value} />
    {:else if Option.isSome(transferDetails.data) && Option.isSome(chains.data)}
      {@const transfer = transferDetails.data.value}
      {@const chainsList = chains.data.value}
      {@const sourceChain = getChain(
        chainsList,
        transfer.source_chain.universal_chain_id,
      )}
      {@const destChain = getChain(
        chainsList,
        transfer.destination_chain.universal_chain_id,
      )}

      <div class="space-y-8">
        <!-- Chain and Token Transfer Display -->
        <div class="flex flex-col gap-6">
          <div class="text-2xl pt-6 px-4">
            {#if !settingsStore.showQuoteTokens}
              <TokenComponent
                chain={sourceChain.value}
                denom={transfer.base_token}
                amount={transfer.base_amount}
              />
            {/if}
          </div>
          <section class="flex flex-col px-4">
            <Label>From</Label>
            {#if Option.isSome(sourceChain)}
                {#if settingsStore.showQuoteTokens}
                  <TokenComponent
                    chain={sourceChain.value}
                    denom={transfer.base_token}
                    amount={transfer.base_amount}
                  />
                {/if}
                {#if Option.isSome(sourceChain)}
                  <ChainComponent chain={sourceChain.value} />
                  <AddressComponent
                    address={transfer.sender_canonical}
                    chain={sourceChain.value}
                    class="text-zinc-400"
                  />
                {:else}
                  <div>{transfer.source_chain.chain_id}</div>
                  <div class="font-mono text-sm text-zinc-400">
                    {transfer.sender_canonical}
                  </div>
                {/if}
            {/if}
            <DateTimeComponent
              class="text-sm text-zinc-400"              
              value={transfer.transfer_send_timestamp}
              showSeconds={false}
            />
            </section>


            

            <section class="flex flex-col px-4">
            <Label>To</Label>
            {#if settingsStore.showQuoteTokens && Option.isSome(destChain)}
              <TokenComponent
                chain={destChain.value}
                denom={transfer.quote_token}
                amount={transfer.quote_amount}
              />
            {/if}
            {#if Option.isSome(destChain)}
              <ChainComponent chain={destChain.value} />
              <AddressComponent
                address={transfer.receiver_canonical}
                chain={destChain.value}
                class="text-zinc-400"
              />
            {:else}
              <div>{transfer.destination_chain.chain_id}</div>
              <div class="font-mono text-sm text-zinc-400">
                {transfer.receiver_canonical}
              </div>
            {/if}
            {#if Option.isSome(transfer.transfer_recv_timestamp)}
              <DateTimeComponent
                class="text-sm text-zinc-400"              
                value={transfer.transfer_recv_timestamp.value}
                showSeconds={false}
              />
            {/if}
            </section>
        </div>

        <PacketTracesComponent packetTraces={transfer.traces}/>
      </div>
    {/if}
    </div>
  </Card>

  <!-- Packet Details Card with toggle button -->
  <div>
    <button 
      class="flex items-center justify-center w-full gap-2 py-2 px-4 text-left hover:text-zinc-300 text-zinc-400 cursor-pointer transition-colors"
      on:click={() => showPacketDetails = !showPacketDetails}
    >
      <span>Packet Details</span>
      <span class="transition-transform duration-300" style={showPacketDetails ? "transform: rotate(180deg)" : ""}>↓</span>
    </button>
    
    {#if showPacketDetails}
      <Card divided transition={false}>
        <PacketComponent />
      </Card>
    {/if}
  </div>
</Sections>
