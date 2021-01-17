import React from 'react';
import {Button, Card, Grid, Message, Modal, Form, Label, Container} from 'semantic-ui-react';

import KittyAvatar from './KittyAvatar';
import {TxButton} from './substrate-lib/components';
import NodeInfo from "./NodeInfo";
import Metadata from "./Metadata";
import BlockNumber from "./BlockNumber";

// --- About Modal ---

const TransferModal = props => {
    const {kitty, accountPair, setStatus} = props;
    const [open, setOpen] = React.useState(false);
    const [formValue, setFormValue] = React.useState({});

    const formChange = key => (ev, el) => {
        /* TODO: 加代码 */
        setFormValue(el.value);
    };

    const confirmAndClose = (unsub) => {
        unsub();
        setOpen(false);
    };

    console.log("accountAddress::",accountPair.address);
    if (kitty.owner == accountPair.address) {
        return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
                      trigger={<Button basic color='blue'>转让</Button>}>
            <Modal.Header>毛孩转让</Modal.Header>
            <Modal.Content>
                <Form>
                    <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id}/>
                    <Form.Input fluid label='转让对象' placeholder='对方地址' onChange={(_, { value }) => setFormValue(value)}/>
                </Form>
            </Modal.Content>
            <Modal.Actions>
                <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
                <TxButton
                    accountPair={accountPair} label='确认转让' type='SIGNED-TX' setStatus={setStatus}
                    onClick={confirmAndClose}
                    attrs={{
                        palletRpc: 'kittiesModule',
                        callable: 'transfer',
                        inputParams: [formValue, kitty.id],
                        paramFields: [true, true]
                    }}
                />
            </Modal.Actions>
        </Modal>;
    }else{
        return null;
    }

};


const KittyLabel = props =>{
    const {kitty, accountPair, setStatus} = props;
    if (kitty.owner == accountPair.address) {
        return (
            <Label style={{backgroundColor:'green'}}>
                我的！！！
            </Label>
        )
    }else {
        return null;
    }
}
// --- About Kitty Card ---

const KittyCard = props => {
    /*
      TODO: 加代码。这里会 UI 显示一张 `KittyCard` 是怎么样的。这里会用到：
      ```
      <KittyAvatar dna={dna} /> - 来描绘一只猫咪
      <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/> - 来作转让的弹出层
      ```
    */
    const {kitty, accountPair, setStatus} = props;
    console.log(kitty.dna)
    return (
        <Card>
            <Card.Content>
                <KittyLabel kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
                <KittyAvatar dna={kitty.dna}/>
                <Message>
                    <Card.Header>ID号:{kitty.id}</Card.Header><br/>
                    <Card.Meta style={{overflowWrap: 'break-word'}}>基因:{kitty.dna.join(",")}</Card.Meta><br/>
                    <span style={{overflowWrap: 'break-word'}}>猫奴:{kitty.owner}</span><br/>
                    <span>{kitty.price}</span><br/>
                </Message>
            </Card.Content>
            <Card.Content extra>
                <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/>
            </Card.Content>
        </Card>
    );
};

const KittyCards = props => {
    const {kitties, accountPair, setStatus} = props;
    /* TODO: 加代码。这里会枚举所有的 `KittyCard` */
    if (kitties.length > 0) {
        // return <KittyCard kitty={kitties[0]} accountPair={accountPair} setStatus={setStatus}/>
        return (
            <Grid stackable columns='equal'>
                {
                    kitties.map(item => {
                        return (
                            <Grid.Column width={5}>
                                <KittyCard kitty={item} accountPair={accountPair} setStatus={setStatus}/>
                            </Grid.Column>
                        )
                    })
                }
            </Grid>
        );
    } else {
        return null;
    }
};

export default KittyCards;
