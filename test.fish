#!/usr/bin/env fish

./release_build.fish

function test_echo
    printf "echoコマンドをテストしています……"
    ln -s ./target/release/rpc echo
    set list Hi! my name is Satori

    for txt in list
        set result (./echo $txt | string sub -e -1)
        if test "$result" != "$txt"
            printf "失敗\n"
            printf "期待: %s\n結果: %s\n" $txt $result
            rm echo
            return 1
        end
    end

    set result (./echo $list | string sub -e -1)
    if test "$result" != "$list"
        printf "失敗\n"
        printf "期待: %s\n結果: %s\n" $txt $result
        rm echo
        return 1
    end

    printf "成功！\n"
    rm echo
end

test_echo