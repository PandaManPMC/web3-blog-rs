// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

contract web3Blog{

    address public owner;

    mapping(string => address) public map_ticket;

    constructor(){
        owner = msg.sender;
    }


    /*
    *   @dev 支付发票 收费 0.01
    *   @params string ticket 票号
    */
    function payTicket(string memory ticket) external payable {
        require(msg.value >= 0.01 ether);
        map_ticket[ticket] = msg.sender;
    }

    function getOwner() public view returns(address) {
        return owner;
    }

    function getAddress(string memory ticket) public view returns(address) {
        return map_ticket[ticket];
    }

    function withdraw() external {
        if (msg.sender == owner){
            address payable pay = payable(owner);
            pay.transfer(payable(this).balance);
        }
    }

    event Received(address, uint);
    receive() external payable {
        emit Received(msg.sender, msg.value);
    }

}