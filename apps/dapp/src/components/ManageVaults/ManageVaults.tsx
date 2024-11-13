import React from "react"
import {
  Button,
  Grid,
  GridItem,
  IconButton,
  Input,
  Stack,
} from "@chakra-ui/react"
import AllVaults from "./AllVaults"
import { useState } from "react"
import { DeployVault } from "../DeployVault/DeployVault"
import { useAppDispatch, useAppSelector } from "@/store/lib/storeHooks"
import { InteractWithVault } from "../InteractWithVault/InteractWithVault"
import { setSelectedVault } from "@/store/lib/features/walletStore"
import ConnectButton from "../Wallet/ConnectButton"
import { useSorobanReact } from "@soroban-react/core"
import { VaultMethod } from "@/hooks/useVault"
import { InputGroup } from "../ui/input-group"
import { DialogBackdrop, DialogRoot, DialogTrigger } from "../ui/dialog"
import { CiSearch } from "react-icons/ci";
import { InspectVault } from "./InspectVault"
import { Strategy, VaultData } from "@/store/lib/types"

export const ManageVaults = () => {
  const { address } = useSorobanReact()
  const [modalStatus, setModalStatus] = useState<{
    deployVault: {
      isOpen: boolean
    },
    interact: {
      isOpen: boolean
    },
    inspect: {
      isOpen: boolean
    }
  }>({
    deployVault: {
      isOpen: false
    },
    interact: {
      isOpen: false
    },
    inspect: {
      isOpen: false
    }
  })
  const dispatch = useAppDispatch()
  const vaults: VaultData[] = useAppSelector(state => state.wallet.vaults.createdVaults)
  const handleInspectVault = async (value: boolean, args?: any) => {
    await dispatch(setSelectedVault({ ...args }))
    setModalStatus({ ...modalStatus, inspect: { isOpen: value } })
  }
  const handleOpenDeployVault = async (method: string, value: boolean, args?: any) => {
    switch (method) {
      case 'create_vault':
        //await dispatch(resetStrategies())
        setModalStatus({ ...modalStatus, deployVault: { isOpen: value } })
        break
      case 'edit_vault':
        //await dispatch(resetStrategies())
        const selectedVault = vaults.find(vault => vault.address === args.address)
        if (!selectedVault) return;
        for (const item of selectedVault.assets) {
        //const newStrategy: Strategy = { ...item, share: selectedVault.strategies.length > 1 ? 100 / selectedVault.strategies.length : 100 };
        //await dispatch(pushStrategy(newStrategy))
        }
        setModalStatus({ ...modalStatus, deployVault: { isOpen: value } })
        break
    }
  }

  const handleOpenInteract = async (method: string, args?: any) => {
    switch (method) {
      case VaultMethod.DEPOSIT:
        await setModalStatus({ ...modalStatus, interact: { isOpen: true } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.DEPOSIT }))
        console.log(args)
        break
      case VaultMethod.WITHDRAW:
        await setModalStatus({ ...modalStatus, interact: { isOpen: true } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.WITHDRAW }))
        console.log(args)
        break
      case VaultMethod.EMERGENCY_WITHDRAW:
        await setModalStatus({ ...modalStatus, interact: { isOpen: true } })
        await dispatch(setSelectedVault({ ...args, method: VaultMethod.EMERGENCY_WITHDRAW }))
        console.log(args)
        break
    }
  }


  return (
    <>
      <Grid
        boxShadow='dark-lg'
        rounded={16}
        templateColumns={{ base: 'repeat(1, 1fr)', md: 'repeat(12, 1fr)' }}
        gap={4}
        maxW={{ sm: '100%', md: '90%', lg: '80%' }}
        py={6}
      >
        <GridItem
          colStart={{ base: 1, md: 2 }}
          colEnd={{ base: 13, md: 8 }}>
          <Stack>
            <InputGroup
              endElement={
                <IconButton
                  rounded={32}
                  size={'sm'}
                  aria-label="search-Vault"
                  colorScheme="green"
                  variant={'ghost'}>
                  <CiSearch />
                </IconButton>}
            >
              <Input
                placeholder='Vault address'
                boxShadow='md'
                rounded={18}
              />
            </InputGroup>
          </Stack>
        </GridItem>
        <GridItem
          colStart={{ base: 1, md: 8 }}
          colEnd={{ base: 13, md: 12 }}
          justifyItems={'start'}
          display={'flex'}
        >
          <ConnectButton />

          {/* !!address */true && <DialogRoot
            open={modalStatus.deployVault.isOpen}
            onOpenChange={(e) => { handleOpenDeployVault('create_vault', e.open) }}
            size={'lg'}
            placement={'center'}>
            <DialogBackdrop backdropFilter='blur(1px)' />
            <DialogTrigger asChild>
              <Button
                rounded={18}
              >
                Add Vault
              </Button>
            </DialogTrigger>
            <DeployVault />
          </DialogRoot>}
        </GridItem>
        <GridItem colSpan={12} colStart={1} colEnd={13} zIndex={'base'}>
          <DialogRoot
            open={modalStatus.interact.isOpen}
            onOpenChange={(e) => { setModalStatus({ ...modalStatus, interact: { isOpen: e.open } }) }}
            size={'lg'}
            placement={'center'}
          >
            <DialogBackdrop backdropFilter='blur(1px)' />
            <InteractWithVault />
          </DialogRoot>
          <AllVaults handleOpenInspect={handleInspectVault} />
        </GridItem>
        <DialogRoot
          open={modalStatus.inspect.isOpen}
          onOpenChange={(e) => { setModalStatus({ ...modalStatus, inspect: { isOpen: e.open } }) }}
          size={'lg'}
          placement={'center'}
        >
          <DialogBackdrop backdropFilter='blur(1px)' />
          <InspectVault
            handleOpenDeployVault={handleOpenDeployVault}
            handleOpenInteract={handleOpenInteract}
            onClose={() => { setModalStatus({ ...modalStatus, inspect: { isOpen: false } }) }}
          />
        </DialogRoot>
      </Grid>
    </>
  )
}

export default ManageVaults