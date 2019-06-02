import React, { useCallback } from "react"
import style from "./styles.scss"
import { timeago } from "~/utils/timeago"

export type TokenProps = {
  token_id: number
  token_partial: string
  created_at: Date
}

const TokenRow: React.FunctionComponent<TokenProps> = props => {
  return (
    <div>
      <div>{props.token_partial}</div>
      <div>{timeago(props.created_at) + " ago"}</div>
      <button>Delete</button>
    </div>
  )
}

export type TokenListProps = {
  tokens: TokenProps[]
}

export const TokenList: React.FunctionComponent<TokenListProps> = props => {
  return (
    <div>
      <p>Token</p>
      <p>Created</p>
      {props.tokens.map((token, idx) => (
        <TokenRow key={idx} {...token} />
      ))}
    </div>
  )
}
