瑪奇小工具
==========

- 介紹
    - 相信有不少覺得瑪奇遊戲的技能繁多，戰鬥按鍵要如何配置而困擾，本專案提供僅使用左手，並且不需大幅移動，就能使用按鍵 1234567890 與 F1~F12
    - 不想用按精的另一個選擇

- 如何啟動？
    - 到 https://github.com/Shaka-chien/Mabinogi-Tools/releases 下載 `mabinogi-tools.exe` 檔案
    - 右鍵該執行檔，使用系統管理員執行
        - ![](./imgs/2025-04-05%2017%2020%2023.png)
    - 出現警告時，依以下操作即可開啟程式
        - ![](./imgs/2025-04-05%2022%2038%2017.png)
        - ![](./imgs/2025-04-05%2022%2038%2031.png)

- 功能(戰鬥模式 `FingingState`)
    - 使用 `ALT` + `12345` 相當於按下 `67890` (EX: `ALT` + `1` = `6`, `ALT` + `2` = `7`, 以此類推)
    - 使用 `ALT` + `F1` = `-`, 使用 `ALT` + `F2` = `=`
    - 使用 `ALT` + `QWER` = `F5` ~ `F8`
    - 使用 `ALT` + `ASDF` = `F9` ~ `F12`
    - 使用 `ALT` + `F3` = `3152`
    - 使用 `ALT` + `F4` = `5231`
    - 按下 `Right Ctrl` (右側的CTRL按鍵): 退回待機模式

- 程式特色
    - 使用 Rust 編譯，效能比美 C++
    - 原始碼看的到，不怕有病毒藏在裡面

- 其他模式
    - 待機模式 `WaitingState`
        - 功能
            - 功能介面
                - 直接使用瑪奇的文字輸入系統，以文字形式與程式互動
            - 注意事項
                - 若有使用中文輸入法，請切換到英文模式，或切換到英文鍵盤之後，再按下 `Right Ctrl` 觸發功能
            - 使用方式有以下2種
                - 按下 L 之後，滑鼠點一下輸入文字框後再按下 `Right Ctrl`
                    - 還不熟悉此程式時, 建議使用此方式
                        - 開啟隊伍
                        - 按下 L 並切換到隊伍對話框
                        - 點一下文字輸入框
                        - 按下 `Right Ctrl`
                    - 不限定一定要開隊伍, 也可以在一般對話中使用
                        - 文字輸入, 不會按下 enter 將文字輸出, 而是在文字框中將文字 `貼上` 告知使用者現在的狀態
                        - 請不要按下 enter 將文字輸出, 不然旁邊的人會覺得你很奇怪！
                - 直接按下 `Right Ctrl`
            - 按下 right ctrl 之後會在對話框中輸入以下文字

                ```
                c: 滑鼠連點, m: 取得滑鼠位置, b: 戰鬥模式, x: 返回待命狀態, q:退出
                ```
            
            - 目的: 可切換到不同的模式, 如滑鼠連點, 或是抓滑鼠位置, 或是進入戰鬥模式
            - `C`: 滑鼠連點模式
            - `M`: 取得滑鼠位置
            - `B`: 戰鬥模式
            - `X`: 回到初始待機模式
            - `Q`: 退出程式
    - 滑鼠連點模式 `MouseClicksState`
        - 進入此模式會直接連點滑鼠左鍵
        - 按下 `Esc` 後會回到 `WaitingState`
    - 取得滑鼠位置 `MousePositionState`
        - 進入此模式會在文字框中顯示當前的滑鼠位置
        - 按下 `Esc` 後會回到 `WaitingState`

- 我想要自己編譯程式，可以嗎？
    - 為何要自己編譯？
        - 若有人對於已編譯的程式不放心，也可自行編譯
        - 若有人有自己的需求修改程式，則可自行編譯
    - 請依以下步驟執行
        - 請安裝 rust 程式 (只需要執行的話不需要安裝 rust)
            - 參考: https://www.rust-lang.org/
            - 找到 `install` 頁面後 下載安裝後即可使用
        - 編譯
            - 使用 管理員權限 開啟 cmd.exe
                - 為何要用 管理員權限 ? 因為我們要直接控制滑鼠鍵盤, 若不使用 管理員權限 的話, 指令不會送到瑪奇的程式
            - 切換到專案路徑
            - 執行以下指令即可編譯

                ```bash
                # debug
                cargo build
                # release
                cargo build --release
                ```
            
            - 編譯 + 執行

                ```bash
                # debug
                cargo run
                # release
                cargo run --release
                ```