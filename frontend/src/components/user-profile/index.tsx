import React, { useCallback } from "react"
import style from "./styles.scss"
import { RemoteData } from "~/utils/remote-data"
import { UserView } from "~/models/api"

type ProfileProps = {
  user: RemoteData<UserView>
}

export const UserProfile: React.FunctionComponent<ProfileProps> = props => {
  return props.user.type === "Done" ? (
    <div className={style["profile"]}>
      <img className={style["avatar"]} src={props.user.data.avatar} />
      <div className={style["text"]}>
        <div className={style["name"]}>{props.user.data.name}</div>
        <div className={style["email"]}>{props.user.data.email}</div>
      </div>
    </div>
  ) : (
    <div />
  )
}
