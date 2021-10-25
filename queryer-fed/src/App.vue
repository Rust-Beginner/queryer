<script setup lang="ts">
import { ref, onMounted, Ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api'
import Clipboard from 'clipboard'

class Result {
  readonly data?: string
  readonly err?: string

  constructor (data?: string, err?: string) {
    this.data = data
    this.err = err
  }

  get isInit () {
    return this.data == null && this.err == null
  }

  get table () {
    if (this.data == null || !this.data.trim()) return []
    let rows = this.data.trim().split('\n')
    return rows.map(row => row.split(','))
  }

  get isMsg () {
    return this.isErr || !this.data
  }

  get isErr () {
    return this.err
  }
}

const input = ref('')
const example = ref('')
const copyBtn: Ref<Element | null> = ref(null)
const copyText = ref('copy')
const result = ref(new Result())

const loading = ref(false)
const disableRun = computed(() => !input.value.trim() || loading.value)

invoke('example_sql').then(x => {
  example.value = x as string
})

onMounted(() => {
  if (copyBtn.value) {
    let clipboard = new Clipboard(copyBtn.value)
    clipboard.on('success', e => {
      copyText.value = 'copied!'
      setTimeout(() => {
        copyText.value = 'copy'
      }, 1500)
    })
  }
})

function handleKeyPress (event: KeyboardEvent) {
  if (event.key === 'Enter' && event.ctrlKey) {
    query()
  }
}

function query () {
  if (disableRun.value) return
  loading.value = true
  invoke('query', {
    sql: input.value,
  }).then(x => {
    result.value = new Result(x as string)
  }).catch(e => {
    result.value = new Result(undefined, e)
  }).then(() => {
    loading.value = false
  })
}

</script>

<template>
  <div class="p-4">
    <h1 class="text-2xl text-red-400 italic font-bold">Queryer&nbsp;🧐</h1>

    <div class="relative">
      <textarea
        class="border rounded-md border-red-200 resize-none w-full my-3 h-32 py-1 px-3 outline-none"
        placeholder="input a sql then press ^enter..."
        v-model="input"
        @keypress="handleKeyPress"
      ></textarea>

      <button
        :disabled="disableRun"
        @click="query"
        class="absolute bottom-6 right-2 bg-red-400 text-white rounded-sm px-3 py-1 shadow-md disabled:bg-gray-400 disabled:cursor-not-allowed"
      >{{loading ? 'Running...' : 'Run'}}</button>
    </div>

    <div class="bg-gray-200 py-1 px-3 rounded-sm mb-3">
      Example: <span class="text-gray-500" id="example">{{example}}</span>
      <a ref="copyBtn"
        href="javascript: void 0"
        class="text-red-400 ml-2 font-bold"
        data-clipboard-target="#example"
      >{{copyText}}</a>
    </div>

    <template v-if="!result.isInit">
      <div v-if="result.isMsg"
        class="py-1 px-3 bg-gray-100 rounded-sm"
        :class="[result.isErr ? 'text-red-400 italic' : 'text-gray-700']"
      >
        {{result.err || 'Empty Result'}}
      </div>

      <table v-else class="border-collapse text-xs text-gray-600 text-center">
        <thead>
          <tr>
            <th class="border border-gray-500 p-1" v-for="h in result.table[0]">{{h}}</th>
          </tr>
        </thead>

        <tbody>
          <tr v-for="(row, index) in result.table.slice(1)">
            <td class="border border-gray-500 p-1" :class="[index < 3 && 'bg-red-100']" v-for="t in row">{{t}}</td>
          </tr>
        </tbody>
      </table>
    </template>
  </div>

</template>

<style>

</style>
