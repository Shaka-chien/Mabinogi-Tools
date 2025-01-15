瑪奇小工具
==========

- 介紹
    - 使用 rust 實作, 透過呼叫 winapi 來實現模擬滑鼠鍵盤, 目的在於玩瑪奇時可以輕鬆的玩

- 功能
    - 如何編譯？
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
            
            - 當然也可以直接使用 exe 來執行 (同上)
    
    - 如何使用
        - 初始待機模式 `WaitingState`
            - 程式一開始執行時是這個模式
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
        - 戰鬥模式 `FingingState`
            - 說明
                - 變更按鍵配置, 讓戰鬥時有更多的技能可使用
                - 進入此模式後, 會在文字框中顯示一段文字 `戰鬥狀態開始..., 0.5 秒後關閉此對話框, Right Ctrl 回到 WaitingState` 並於0.5秒後將文字清除
            - 按鍵變更
                - ALT + 12345 = 67890
                - ALT + QWER = F5 - F8
                - ALT + ASDF = F9 - F12
- 其他說明
    - 為何程式中有 `shaka` 的英文？
        - 因為... 我叫 `沙加` 英文就是 `shaka`
    - 如果程式怪怪的，要怎麼強制關閉？
        - 方法1: 切到 cmd.exe 並按下 `CTRL+C` 則可強制退出
        - 方法2: 把 cmd.exe 程式關閉
    - 已知 Bug
        - 滑鼠連點, 有時候會無法退回到 `WaitingState` 原因待查
        - 戰鬥模式, 偶發, 有時候只會觸發 key down, 不會觸發 key up, 發生此狀況時再按一下一樣的快鍵即可
            - 例: ALT + R 要按 F8 (EX: `迴旋斬擊`) 但放開 R 之後 技能還在集氣，沒有發出去
            - 發現此狀況時，再按一次 ALT + R 即可發動該技能
            - 原因不明，待查
