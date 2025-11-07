declare module '@wasm/geolocation' {
    export default function init(): Promise<void>
    export function parse_document_wasm(xml: string): any
    export function parse_nfe_wasm(xml: string): any
    export function parse_cte_wasm(xml: string): any
}
