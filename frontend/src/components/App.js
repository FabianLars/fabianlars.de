import { h } from 'preact'
import { lazy, Suspense } from 'preact/compat'
import { BrowserRouter, Route, Switch, Redirect } from 'react-router-dom'
import AppHeader from './AppHeader'

// Code-splitting is automated for routes
const Home = lazy(() => import('../views/Home'))
const Mods = lazy(() => import('../views/Mods'))
const Showcase = lazy(() => import('../views/Showcase'))
const Blog = lazy(() => import('../views/Blog'))
const Gear = lazy(() => import('../views/Gear'))

export default () => {
    return (
        <div id='app'>
            <BrowserRouter>
                <AppHeader />
                <Suspense fallback={<div>Loading...</div>}>
                    <Switch>
                        <Route exact path='/' component={Home} />
                        <Route path='/mods' component={Mods} />
                        <Route path='/showcase' component={Showcase} />
                        <Route path='/blog' component={Blog} />
                        <Route path='/gear' component={Gear} />
                        <Redirect to='/' />
                    </Switch>
                </Suspense>
            </BrowserRouter>
        </div>
    )
}
