contract solidity {
    function add_one(uint32 i) public pure returns(uint32) {
        return i + 1;
    }

    function echo(uint32 i) public pure returns(uint32) {
        return i;
    }
}