import React, { useCallback } from "react"
import style from "./styles.scss"
import { timeago } from "~/utils/timeago"
import { RemoteData } from "~/utils/remote-data"
import check from "~/img/check.svg"

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
  tokens: RemoteData<TokenProps[]>
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
        {props.tokens.type === "Done" ? (
          props.tokens.data.map((token, idx) => (
            <TokenRow key={idx} {...token} />
          ))
        ) : props.tokens.type === "Started" ? (
          <tr>
            <td className={style["td-info"]} colSpan={3}>
              Loading
            </td>
          </tr>
        ) : (
          <tr>
            <td className={style["td-info"]} colSpan={3}>
              Faild
            </td>
          </tr>
        )}
      </tbody>
    </table>
  )
}

export type TokenDisplayProps = {
  token: string
}

export const TokenDisplay: React.FunctionComponent<
  TokenDisplayProps
> = props => {
  return (
    <div className={style["token-display"]}>
      <div className={style["token-display__top"]}>
        <img src={check} />
        <div className={style["token-display__text"]}>
          <p>Token created.</p>
          <p>It will never be displayed again.</p>
        </div>
      </div>
      <div className={style["token-display__bottom"]}>{props.token}</div>
    </div>
  )
}
