package ontology
import (
	"bytes"
	"encoding/hex"
	"fmt"
	sdkcom "github.com/ontio/ontology-go-sdk/common"
	"github.com/ontio/ontology/common"
	"github.com/ontio/ontology/core/types"
	"github.com/ontio/ontology/smartcontract/states"
)


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
	tx := this.vm.NewDeployWasmVMCodeTransaction(gasPrice, gasLimit, &sdkcom.SmartContract{
		Code:        invokeCode,
		NeedStorage: needStorage,
		Name:        name,
		Version:     version,
		Author:      author,
		Email:       email,
		Description: desc,
	})
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
