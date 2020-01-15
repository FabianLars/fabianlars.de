import { h, cloneElement, Fragment } from 'preact'
import {
    AppBar,
    Toolbar,
    Typography,
    Tab,
    Tabs,
    useScrollTrigger,
    Hidden,
    IconButton,
    Drawer,
    List,
    ListItem,
    ListItemText,
} from '@material-ui/core'
import { makeStyles } from '@material-ui/styles'
import MenuIcon from '@material-ui/icons/Menu'
import { useState } from 'preact/hooks'
import { Link } from 'react-router-dom'

const useStyles = makeStyles(theme => ({
    toolbar: {
        flexWrap: 'wrap',
    },
    toolbarTitle: {
        flexGrow: 1,
    },
    menuButton: {
        marginRight: theme.spacing(2),
    },
    list: {
        width: 250,
    },
    appHeaderPlaceholder: {
        marginBottom: theme.spacing(1.5),
    },
}))

export default () => {
    const classes = useStyles()
    const [value, setValue] = useState(0)
    const [open, setOpen] = useState(false)

    const handleChange = (event, newValue) => {
        setValue(newValue)
    }

    const toggleDrawer = () => event => {
        if (
            event.type === 'keydown' &&
            (event.key === 'Tab' || event.key === 'Shift')
        )
            return
        setOpen(!open)
    }

    return (
        <Fragment>
            <Hidden
                smDown
                children={
                    <Fragment>
                        <ElevationScroll>
                            <AppBar color='default'>
                                <Toolbar class={classes.toolbar}>
                                    <Typography
                                        variant='h6'
                                        color='inherit'
                                        noWrap
                                        class={classes.toolbarTitle}
                                    >
                                        FabianLars
                                    </Typography>
                                    <Tabs
                                        variant='fullWidth'
                                        value={value}
                                        onChange={handleChange}
                                    >
                                        <Tab
                                            component={Link}
                                            to='/'
                                            label='Home'
                                        />
                                        <Tab
                                            component={Link}
                                            to='/mods'
                                            label='Mods'
                                        />
                                        <Tab
                                            component={Link}
                                            to='/showcase'
                                            label='Showcase'
                                        />
                                        <Tab
                                            component={Link}
                                            to='/blog'
                                            label='Blog'
                                        />
                                    </Tabs>
                                </Toolbar>
                            </AppBar>
                        </ElevationScroll>
                        <Toolbar class={classes.appHeaderPlaceholder} />
                    </Fragment>
                }
            />
            <Hidden
                mdUp
                children={
                    <Fragment>
                        <ElevationScroll>
                            <AppBar color='default'>
                                <Toolbar class={classes.toolbar}>
                                    <IconButton
                                        edge='start'
                                        class={classes.menuButton}
                                        color='inherit'
                                        aria-label='Menu'
                                        onClick={toggleDrawer()}
                                    >
                                        <MenuIcon />
                                    </IconButton>
                                    <Typography
                                        variant='h6'
                                        color='inherit'
                                        noWrap
                                        class={classes.toolbarTitle}
                                    >
                                        FabianLars
                                    </Typography>
                                </Toolbar>
                            </AppBar>
                        </ElevationScroll>
                        <Toolbar class={classes.appHeaderPlaceholder} />

                        <Drawer open={open} onClose={toggleDrawer()}>
                            <div
                                class={classes.list}
                                role='presentation'
                                onClick={toggleDrawer()}
                                onKeyDown={toggleDrawer()}
                            >
                                <List>
                                    <ListItem component={Link} button to='/'>
                                        <ListItemText primary='Home' />
                                    </ListItem>
                                    <ListItem
                                        component={Link}
                                        button
                                        to='/mods'
                                    >
                                        <ListItemText primary='Mods' />
                                    </ListItem>
                                    <ListItem
                                        component={Link}
                                        button
                                        to='/showcase'
                                    >
                                        <ListItemText primary='Showcase' />
                                    </ListItem>
                                    <ListItem
                                        component={Link}
                                        button
                                        to='/blog'
                                    >
                                        <ListItemText primary='Blog' />
                                    </ListItem>
                                </List>
                            </div>
                        </Drawer>
                    </Fragment>
                }
            />
        </Fragment>
    )
}

const ElevationScroll = props => {
    const trigger = useScrollTrigger({
        disableHysteresis: true,
        threshold: 0,
    })

    return cloneElement(props.children, {
        elevation: trigger ? 4 : 0,
    })
}
