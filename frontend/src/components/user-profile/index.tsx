import React, { useCallback } from "react"
import style from "./styles.scss"
import { RemoteData } from "~/utils/remote-data"
import { UserView } from "~/api"

type ProfileProps = {
  user: UserView
}

export const UserProfile: React.FunctionComponent<ProfileProps> = props => {
  return (
    <div className={style["profile"]}>
      <img className={style["avatar"]} src={props.user.avatar} />
      <div className={style["text"]}>
        <div className={style["name"]}>{props.user.name}</div>
        <div className={style["email"]}>{props.user.email}</div>
      </div>
    </div>
  )
}
