package ontology
import (
	"bytes"
	"encoding/hex"
	"fmt"
	"github.com/ontio/ontology/core/payload"
	sdkcom "github.com/ontio/ontology-go-sdk/common"
	"github.com/ontio/ontology/common"
	"github.com/ontio/ontology/core/types"
	"github.com/ontio/ontology/smartcontract/states"
)

const VERSION_TRANSACTION = byte(0)

type DemoContract struct {
	contractAddr common.Address
	vm WasmVMContract
	gasPrice uint64
	gasLimit uint64
	signer *Account
	version byte
}

func(this *DemoContract) Deploy(gasPrice, gasLimit uint64,
	singer *Account,
	needStorage byte,
	code,
	name,
	version,
	author,
	email,
	desc string) (*types.MutableTransaction, error) {
	invokeCode, err := hex.DecodeString(code)
	if err != nil {
		return nil, fmt.Errorf("code hex decode error:%s", err)
	}
	deployPayload := &payload.DeployCode{
    		Code:        invokeCode,
    		NeedStorage: needStorage,
    		Name:        name,
    		Version:     version,
    		Author:      author,
    		Email:       email,
    		Description: desc,
    	}
    	tx := &types.MutableTransaction{
    		Version:  VERSION_TRANSACTION,
    		TxType:   types.Deploy,
    		Nonce:    uint32(time.Now().Unix()),
    		Payload:  deployPayload,
    		GasPrice: gasPrice,
    		GasLimit: gasLimit,
    		Sigs:     make([]types.Sig, 0, 0),
    	}
	return tx, nil
}

func (this *DemoContract) buildParams(method string, params []interface{}) ([]byte, error) {
	contract := &states.ContractInvokeParam{}
	contract.Address = this.contractAddr
	contract.Method = method
	contract.Version = this.version

	argbytes, err := buildWasmContractParam(params)

	if err != nil {
		return nil, fmt.Errorf("build wasm contract param failed:%s", err)
	}
	contract.Args = argbytes
	bf := bytes.NewBuffer(nil)
	contract.Serialize(bf)
	return bf.Bytes(), nil
}
