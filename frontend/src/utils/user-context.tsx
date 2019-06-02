import React, { createContext, useReducer } from "react"
import { UserView, show_user_self } from "~/models/api"

type UserContext = {
  user?: UserView
  fetchUser: () => void
}

const { Provider, Consumer } = createContext<UserContext>({
  user: undefined,
  fetchUser: () => undefined,
})

export class UserProvider extends React.Component {
  componentDidMount() {
    this.fetchUser()
  }

  fetchUser() {
    let self = this
    async function load() {
      let user = await show_user_self()
      self.setState({ user: user == null ? undefined : user })
    }
    load()
  }

  state = {
    user: undefined,
  }

  render() {
    return (
      <Provider
        value={{ user: this.state.user, fetchUser: this.fetchUser.bind(this) }}
      >
        {this.props.children}
      </Provider>
    )
  }
}

export const UserConsumer = Consumer
