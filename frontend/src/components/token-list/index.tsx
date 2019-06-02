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
    <tr>
      <td>{props.token_partial}</td>
      <td>{timeago(props.created_at) + " ago"}</td>
      <td>
        <button className="button is-black">Delete</button>
      </td>
    </tr>
  )
}

export type TokenListProps = {
  tokens: TokenProps[]
}

export const TokenList: React.FunctionComponent<TokenListProps> = props => {
  return (
    <table className={style["token-table"]}>
      <thead>
        <tr>
          <th>Token</th>
          <th>Created</th>
          <th />
        </tr>
      </thead>
      <tbody>
        {props.tokens.map((token, idx) => (
          <TokenRow key={idx} {...token} />
        ))}
      </tbody>
    </table>
    // <td className={style.list}>
    //   <p className={style["title-1"]}>Token</p>
    //   <p className={style["title-2"]}>Created</p>
    //   {props.tokens.map((token, idx) => (
    //     <TokenRow key={idx} {...token} />
    //   ))}
    // </div>
  )
}
