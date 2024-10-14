import * as React from 'react'
import Avatar from '@mui/joy/Avatar'
import Chip from '@mui/joy/Chip'
import IconButton from '@mui/joy/IconButton'
import Stack from '@mui/joy/Stack'
import Typography from '@mui/joy/Typography'
import CircleIcon from '@mui/icons-material/Circle'
import IndexIcon from '@mui/icons-material/SortRounded'
import SettingsIcon from '@mui/icons-material/SettingsRounded'
import HistoryRoundIcon from '@mui/icons-material/HistoryRounded'
import AddRoundIcon from '@mui/icons-material/AddRounded'
import LoginIcon from '@mui/icons-material/LoginRounded'

import Tooltip from '@mui/joy/Tooltip'
import { useNavigate } from 'react-router-dom'
import { useCopilotContext } from '../Context'

type ChatHeaderProps = {}

export default function ChatHeader(props: ChatHeaderProps) {
    const { repo, setMessages, setThreadId, user } = useCopilotContext()
    const navigate = useNavigate()
    return (
        <Stack
            direction="row"
            justifyContent="space-between"
            sx={{}}
            py={1}
            px={1}
        >
            <Stack
                direction="row"
                spacing={{ xs: 1, md: 2 }}
                alignItems="center"
            >
                {repo?.name && (
                    <>
                        {repo?.avatarUrl ? (
                            <Avatar size="md" src={repo?.avatarUrl} />
                        ) : (
                            <Avatar size="md" />
                        )}

                        <div>
                            <Typography
                                fontWeight="lg"
                                fontSize="lg"
                                component="h2"
                                noWrap
                                endDecorator={
                                    repo?.status?.lastIndexedCommit ? (
                                        <Chip
                                            variant="outlined"
                                            size="sm"
                                            color="neutral"
                                            sx={{
                                                borderRadius: 'sm',
                                            }}
                                            startDecorator={
                                                <CircleIcon
                                                    sx={{ fontSize: 8 }}
                                                    color="success"
                                                />
                                            }
                                            slotProps={{
                                                root: { component: 'span' },
                                            }}
                                        >
                                            {repo.status.lastIndexedCommit.substring(
                                                0,
                                                7,
                                            )}
                                        </Chip>
                                    ) : repo?.status?.isIndexing ? (
                                        <>
                                            <Chip
                                                variant="outlined"
                                                size="sm"
                                                color="neutral"
                                                sx={{
                                                    borderRadius: 'sm',
                                                }}
                                                startDecorator={
                                                    <CircleIcon
                                                        sx={{ fontSize: 8 }}
                                                        color="warning"
                                                    />
                                                }
                                                slotProps={{
                                                    root: { component: 'span' },
                                                }}
                                            >
                                                Running
                                            </Chip>
                                        </>
                                    ) : (
                                        <Chip
                                            variant="outlined"
                                            size="sm"
                                            color="neutral"
                                            sx={{
                                                borderRadius: 'sm',
                                            }}
                                            startDecorator={
                                                <CircleIcon
                                                    sx={{ fontSize: 8 }}
                                                    color="warning"
                                                />
                                            }
                                            slotProps={{
                                                root: { component: 'span' },
                                            }}
                                        >
                                            Index Unavailable
                                        </Chip>
                                    )
                                }
                            >
                                {repo?.name}
                            </Typography>
                            <Typography level="body-sm">
                                {repo?.owner}
                            </Typography>
                        </div>
                    </>
                )}
            </Stack>
            <Stack spacing={1.2} direction="row" alignItems="center">
                <>
                    <Tooltip
                        title="New Thread"
                        size="sm"
                        variant="outlined"
                        component="main"
                    >
                        <IconButton
                            color="neutral"
                            variant="outlined"
                            size="sm"
                            component="main"
                            onClick={() => {
                                setThreadId('')
                                setMessages([])
                                navigate('message')
                            }}
                        >
                            <AddRoundIcon />
                        </IconButton>
                    </Tooltip>
                    <Tooltip
                        title="History"
                        size="sm"
                        variant="outlined"
                        component="main"
                    >
                        <IconButton
                            color="neutral"
                            variant="outlined"
                            size="sm"
                            sx={{
                                display: 'inline-flex',
                            }}
                            onClick={() => {
                                navigate('history')
                            }}
                        >
                            <HistoryRoundIcon />
                        </IconButton>
                    </Tooltip>

                    <Tooltip
                        title="Indexing"
                        size="sm"
                        variant="outlined"
                        component="main"
                    >
                        <IconButton
                            color="neutral"
                            variant="outlined"
                            size="sm"
                            sx={{
                                display: 'inline-flex',
                            }}
                            onClick={() => {
                                if (repo) {
                                    navigate('/chat/index')
                                }
                            }}
                        >
                            <IndexIcon />
                        </IconButton>
                    </Tooltip>
                </>
                <Tooltip
                    title={user ? 'Settings' : 'Login'}
                    size="sm"
                    variant="outlined"
                    component="main"
                >
                    <IconButton
                        color='neutral'
                        variant="outlined"
                        size="sm"
                        onClick={() => navigate('settings')}
                    >
                        {user ? <SettingsIcon /> : <LoginIcon />}
                    </IconButton>
                </Tooltip>
            </Stack>
        </Stack>
    )
}