(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func (param i32 i32) (result i32)))
  (type (;5;) (func (param i32 i32 i32)))
  (type (;6;) (func (result i32)))
  (type (;7;) (func (param i32 i32 i32 i32)))
  (type (;8;) (func (param i32 i32 i32) (result i32)))
  (type (;9;) (func (param i32 i32 i32 i32) (result i32)))
  (type (;10;) (func (param i32 i32 i32 i32 i32)))
  (type (;11;) (func (param i32 i32 i32 i64 i32)))
  (type (;12;) (func (param i32 i64)))
  (type (;13;) (func (param i64)))
  (type (;14;) (func (param i32 i32 i64 i32 i32) (result i32)))
  (type (;15;) (func (param i64) (result i32)))
  (type (;16;) (func (param i32) (result i64)))
  (type (;17;) (func (param i32 i32) (result i64)))
  (import "env" "checkNoPayment" (func (;0;) (type 1)))
  (import "env" "bigIntNew" (func (;1;) (type 15)))
  (import "env" "bigIntGetUnsignedArgument" (func (;2;) (type 0)))
  (import "env" "mBufferNewFromBytes" (func (;3;) (type 4)))
  (import "env" "mBufferStorageStore" (func (;4;) (type 4)))
  (import "env" "mBufferNew" (func (;5;) (type 6)))
  (import "env" "getNumArguments" (func (;6;) (type 6)))
  (import "env" "mBufferGetLength" (func (;7;) (type 3)))
  (import "env" "smallIntGetUnsignedArgument" (func (;8;) (type 16)))
  (import "env" "signalError" (func (;9;) (type 0)))
  (import "env" "mBufferGetBytes" (func (;10;) (type 4)))
  (import "env" "bigIntGetDCDTExternalBalance" (func (;11;) (type 11)))
  (import "env" "bigIntGetExternalBalance" (func (;12;) (type 0)))
  (import "env" "managedTransferValueExecute" (func (;13;) (type 14)))
  (import "env" "mBufferFinish" (func (;14;) (type 3)))
  (import "env" "smallIntFinishUnsigned" (func (;15;) (type 13)))
  (import "env" "bigIntFinishUnsigned" (func (;16;) (type 2)))
  (import "env" "getNumDCDTTransfers" (func (;17;) (type 6)))
  (import "env" "bigIntGetCallValue" (func (;18;) (type 2)))
  (import "env" "bigIntCmp" (func (;19;) (type 4)))
  (import "env" "mBufferAppend" (func (;20;) (type 4)))
  (import "env" "finish" (func (;21;) (type 0)))
  (import "env" "managedSCAddress" (func (;22;) (type 2)))
  (import "env" "keccak256" (func (;23;) (type 8)))
  (import "env" "mBufferToBigIntUnsigned" (func (;24;) (type 4)))
  (import "env" "mBufferStorageLoad" (func (;25;) (type 4)))
  (import "env" "managedAsyncCall" (func (;26;) (type 7)))
  (import "env" "managedCaller" (func (;27;) (type 2)))
  (import "env" "managedOwnerAddress" (func (;28;) (type 2)))
  (import "env" "mBufferEq" (func (;29;) (type 4)))
  (import "env" "managedSignalError" (func (;30;) (type 2)))
  (import "env" "mBufferAppendBytes" (func (;31;) (type 8)))
  (import "env" "managedGetOriginalTxHash" (func (;32;) (type 2)))
  (import "env" "mBufferSetBytes" (func (;33;) (type 8)))
  (import "env" "mBufferFromBigIntUnsigned" (func (;34;) (type 4)))
  (import "env" "mBufferGetArgument" (func (;35;) (type 4)))
  (import "env" "mBufferCopyByteSlice" (func (;36;) (type 9)))
  (import "env" "mBufferGetByteSlice" (func (;37;) (type 9)))
  (func (;38;) (type 1)
    (local i32)
    call 0
    i32.const 1
    call 39
    i32.const 0
    i64.const 0
    call 1
    local.tee 0
    call 2
    i32.const 1048898
    i32.const 17
    call 3
    local.get 0
    call 40
    call 4
    drop)
  (func (;39;) (type 2) (param i32)
    call 6
    local.get 0
    i32.eq
    if  ;; label = @1
      return
    end
    i32.const 1048764
    i32.const 25
    call 9
    unreachable)
  (func (;40;) (type 3) (param i32) (result i32)
    (local i32)
    call 5
    local.tee 1
    local.get 0
    call 34
    drop
    local.get 1)
  (func (;41;) (type 1)
    (local i32 i32 i32 i32 i32 i32 i32 i64)
    global.get 0
    i32.const 96
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                call 42
                local.tee 4
                call 43
                local.tee 2
                call 44
                br_if 0 (;@6;)
                local.get 2
                call 45
                local.set 2
                local.get 0
                i32.const 25
                i32.store offset=68
                local.get 0
                i32.const 1049110
                i32.store offset=64
                local.get 0
                i32.const 24
                i32.add
                local.tee 1
                local.get 2
                call 46
                local.get 1
                call 47
                local.set 2
                local.get 1
                call 48
                local.set 1
                call 5
                local.set 3
                loop  ;; label = @7
                  local.get 1
                  if  ;; label = @8
                    local.get 3
                    local.get 0
                    i32.const 24
                    i32.add
                    call 47
                    call 49
                    local.get 1
                    i32.const -1
                    i32.add
                    local.set 1
                    br 1 (;@7;)
                  end
                end
                local.get 0
                i32.load offset=40
                local.get 0
                i32.load offset=36
                i32.ne
                br_if 1 (;@5;)
                local.get 0
                i32.load8_u offset=32
                if  ;; label = @7
                  i32.const 1049400
                  i32.const 0
                  i32.store
                  i32.const 1059404
                  i32.const 0
                  i32.store8
                end
                local.get 4
                call 5
                call 4
                drop
                local.get 0
                call 6
                i32.store offset=20
                local.get 0
                i32.const 0
                i32.store offset=16
                local.get 0
                i32.const 88
                i32.add
                local.tee 4
                i64.const 0
                i64.store
                local.get 0
                i32.const 80
                i32.add
                local.tee 5
                i64.const 0
                i64.store
                local.get 0
                i32.const 72
                i32.add
                local.tee 6
                i64.const 0
                i64.store
                local.get 0
                i64.const 0
                i64.store offset=64
                local.get 0
                i32.const 8
                i32.add
                local.get 0
                i32.const -64
                i32.sub
                local.get 2
                call 7
                local.tee 1
                call 50
                local.get 2
                i32.const 0
                local.get 0
                i32.load offset=8
                local.get 0
                i32.load offset=12
                call 51
                drop
                local.get 0
                i32.const 52
                i32.add
                local.get 4
                i64.load
                i64.store align=4
                local.get 0
                i32.const 44
                i32.add
                local.get 5
                i64.load
                i64.store align=4
                local.get 0
                i32.const 36
                i32.add
                local.get 6
                i64.load
                i64.store align=4
                local.get 0
                local.get 0
                i64.load offset=64
                i64.store offset=28 align=4
                local.get 1
                i32.eqz
                br_if 0 (;@6;)
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 1
                    i32.const 22
                    i32.ne
                    br_if 0 (;@8;)
                    local.get 0
                    i32.const 24
                    i32.add
                    i32.const 4
                    i32.or
                    i32.const 1048935
                    i32.const 22
                    call 128
                    br_if 0 (;@8;)
                    local.get 3
                    call 52
                    local.get 0
                    i32.const 0
                    i32.store offset=64
                    local.get 3
                    i32.const 0
                    local.get 0
                    i32.const -64
                    i32.sub
                    i32.const 4
                    call 51
                    br_if 4 (;@4;)
                    local.get 0
                    i32.load offset=64
                    local.tee 1
                    i32.const 24
                    i32.shl
                    local.get 1
                    i32.const 8
                    i32.shl
                    i32.const 16711680
                    i32.and
                    i32.or
                    local.get 1
                    i32.const 8
                    i32.shr_u
                    i32.const 65280
                    i32.and
                    local.get 1
                    i32.const 24
                    i32.shr_u
                    i32.or
                    i32.or
                    call 45
                    call 45
                    local.tee 3
                    call 7
                    i32.const 32
                    i32.ne
                    br_if 5 (;@3;)
                    local.get 0
                    i32.const 16
                    i32.add
                    call 53
                    call 8
                    local.tee 7
                    i64.const 4294967296
                    i64.ge_u
                    br_if 6 (;@2;)
                    i32.const 0
                    local.set 1
                    local.get 7
                    i32.wrap_i64
                    if  ;; label = @9
                      block  ;; label = @10
                        local.get 0
                        i32.load offset=16
                        local.get 0
                        i32.load offset=20
                        i32.ge_s
                        if  ;; label = @11
                          call 5
                          drop
                          br 1 (;@10;)
                        end
                        local.get 0
                        i32.const 16
                        i32.add
                        call 53
                        call 54
                        drop
                      end
                      i32.const 1
                      local.set 1
                    end
                    local.get 0
                    i32.const 0
                    i32.store offset=68
                    local.get 0
                    i32.const 1049384
                    i32.store offset=64
                    i32.const 2
                    i32.ge_u
                    br_if 7 (;@1;)
                    local.get 0
                    i32.const 0
                    i32.store offset=68
                    local.get 0
                    i32.const 1049384
                    i32.store offset=64
                    local.get 0
                    i32.load offset=16
                    local.get 0
                    i32.load offset=20
                    i32.lt_s
                    br_if 7 (;@1;)
                    local.get 1
                    br_if 1 (;@7;)
                    local.get 0
                    local.get 3
                    call 55
                    local.get 0
                    i32.load
                    i32.const 1
                    i32.eq
                    if  ;; label = @9
                      local.get 3
                      i32.const 2
                      local.get 0
                      i32.load offset=4
                      call 56
                      br 3 (;@6;)
                    end
                    local.get 3
                    i32.const 0
                    local.get 1
                    call 56
                    br 2 (;@6;)
                  end
                  i32.const 1048975
                  i32.const 54
                  call 9
                  unreachable
                end
                local.get 3
                i32.const 0
                local.get 1
                call 56
              end
              local.get 0
              i32.const 96
              i32.add
              global.set 0
              return
            end
            local.get 0
            i32.const -64
            i32.sub
            i32.const 1049042
            i32.const 14
            call 57
            unreachable
          end
          i32.const 1048957
          i32.const 12
          i32.const 1049135
          i32.const 17
          call 58
          unreachable
        end
        i32.const 1048957
        i32.const 12
        i32.const 1049361
        i32.const 16
        call 58
        unreachable
      end
      i32.const 1048969
      i32.const 6
      i32.const 1049042
      i32.const 14
      call 58
      unreachable
    end
    local.get 0
    i32.const -64
    i32.sub
    i32.const 1049056
    i32.const 18
    call 57
    unreachable)
  (func (;42;) (type 6) (result i32)
    (local i32 i32)
    call 5
    local.tee 0
    call 32
    i32.const 1049229
    i32.const 10
    call 3
    local.tee 1
    local.get 0
    call 20
    drop
    local.get 1)
  (func (;43;) (type 3) (param i32) (result i32)
    local.get 0
    call 5
    local.tee 0
    call 25
    drop
    local.get 0)
  (func (;44;) (type 3) (param i32) (result i32)
    local.get 0
    call 7
    i32.eqz)
  (func (;45;) (type 3) (param i32) (result i32)
    (local i32)
    call 5
    local.tee 1
    local.get 0
    call 20
    drop
    local.get 1)
  (func (;46;) (type 0) (param i32 i32)
    (local i32)
    local.get 0
    local.get 1
    call 7
    local.tee 2
    i32.store offset=16
    local.get 0
    i32.const 0
    i32.store offset=12
    local.get 0
    i32.const 0
    i32.store8 offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (func (;47;) (type 3) (param i32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    call 48
    local.set 2
    local.get 1
    i32.const 25
    i32.store offset=12
    local.get 1
    i32.const 1049110
    i32.store offset=8
    local.get 1
    local.get 0
    i32.load
    local.get 0
    i32.load offset=12
    local.tee 3
    local.get 2
    call 102
    local.get 1
    i32.load
    i32.const 1
    i32.ne
    if  ;; label = @1
      local.get 1
      i32.const 8
      i32.add
      i32.const 1049314
      i32.const 15
      call 57
      unreachable
    end
    local.get 1
    i32.load offset=4
    local.get 0
    local.get 2
    local.get 3
    i32.add
    i32.store offset=12
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;48;) (type 3) (param i32) (result i32)
    (local i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    i32.const 25
    i32.store offset=12
    local.get 1
    i32.const 1049110
    i32.store offset=8
    local.get 0
    local.get 0
    i32.load offset=12
    local.get 1
    i32.const 4
    i32.add
    i32.const 4
    call 101
    i32.eqz
    if  ;; label = @1
      local.get 0
      local.get 0
      i32.load offset=12
      i32.const 4
      i32.add
      i32.store offset=12
      loop  ;; label = @2
        local.get 2
        i32.const 4
        i32.eq
        i32.eqz
        if  ;; label = @3
          local.get 1
          i32.const 4
          i32.add
          local.get 2
          i32.add
          local.get 2
          i32.const 1
          i32.add
          local.set 2
          i64.load8_u
          local.get 3
          i64.const 8
          i64.shl
          i64.or
          local.set 3
          br 1 (;@2;)
        end
      end
      local.get 1
      i32.const 16
      i32.add
      global.set 0
      local.get 3
      i32.wrap_i64
      return
    end
    local.get 1
    i32.const 8
    i32.add
    i32.const 1049314
    i32.const 15
    call 57
    unreachable)
  (func (;49;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    local.get 1
    i32.const 24
    i32.shl
    i32.or
    local.get 1
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 1
    i32.const 24
    i32.shr_u
    i32.or
    i32.or
    i32.store offset=12
    local.get 0
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 31
    drop
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;50;) (type 5) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    i32.const 32
    local.get 2
    call 111
    local.get 3
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;51;) (type 9) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 3
    local.get 2
    call 37
    i32.const 0
    i32.ne)
  (func (;52;) (type 3) (param i32) (result i32)
    local.get 0
    call 7
    i32.const 2
    i32.shr_u)
  (func (;53;) (type 3) (param i32) (result i32)
    (local i32)
    local.get 0
    i32.load
    local.tee 1
    local.get 0
    i32.load offset=4
    i32.lt_s
    if  ;; label = @1
      local.get 0
      local.get 1
      i32.const 1
      i32.add
      i32.store
      local.get 1
      return
    end
    i32.const 1048969
    i32.const 6
    i32.const 1049135
    i32.const 17
    call 58
    unreachable)
  (func (;54;) (type 3) (param i32) (result i32)
    local.get 0
    call 5
    local.tee 0
    call 35
    drop
    local.get 0)
  (func (;55;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1048915
    i32.const 11
    call 3
    local.tee 3
    local.get 1
    call 100
    local.get 3
    call 5
    local.tee 1
    call 25
    drop
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            call 7
            i32.eqz
            if  ;; label = @5
              i32.const 0
              local.set 3
              br 1 (;@4;)
            end
            local.get 2
            i32.const 8
            i32.add
            local.tee 4
            local.get 3
            call 43
            call 46
            i32.const 0
            local.set 3
            local.get 2
            i32.const 0
            i32.store8 offset=31
            i32.const 1
            local.set 5
            local.get 4
            local.get 2
            i32.load offset=20
            local.get 2
            i32.const 31
            i32.add
            i32.const 1
            call 101
            br_if 1 (;@3;)
            local.get 2
            i32.load offset=20
            local.tee 6
            i32.const 1
            i32.add
            local.set 4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    local.get 2
                    i32.load8_u offset=31
                    br_table 3 (;@5;) 2 (;@6;) 1 (;@7;) 0 (;@8;)
                  end
                  i32.const 1049029
                  i32.const 13
                  call 93
                  unreachable
                end
                i32.const 2
                local.set 5
              end
              local.get 2
              local.get 2
              i32.load offset=8
              local.get 4
              i32.const 32
              call 102
              local.get 2
              i32.load
              i32.const 1
              i32.ne
              br_if 3 (;@2;)
              local.get 2
              i32.load offset=4
              local.set 1
              local.get 6
              i32.const 33
              i32.add
              local.set 4
              local.get 5
              local.set 3
            end
            local.get 2
            i32.load offset=24
            local.get 4
            i32.ne
            br_if 3 (;@1;)
            local.get 2
            i32.load8_u offset=16
            i32.eqz
            br_if 0 (;@4;)
            i32.const 1049400
            i32.const 0
            i32.store
            i32.const 1059404
            i32.const 0
            i32.store8
          end
          local.get 0
          local.get 1
          i32.store offset=4
          local.get 0
          local.get 3
          i32.store
          local.get 2
          i32.const 32
          i32.add
          global.set 0
          return
        end
        i32.const 1049314
        i32.const 15
        call 93
        unreachable
      end
      i32.const 1049314
      i32.const 15
      call 93
      unreachable
    end
    i32.const 1049042
    i32.const 14
    call 93
    unreachable)
  (func (;56;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    i32.const 1048915
    i32.const 11
    call 3
    local.tee 4
    local.get 0
    call 100
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 1
            i32.const 1
            i32.sub
            br_table 1 (;@3;) 2 (;@2;) 0 (;@4;)
          end
          i32.const 0
          local.get 4
          call 89
          br 2 (;@1;)
        end
        local.get 3
        i32.const 8
        i32.add
        call 103
        local.get 3
        local.get 3
        i32.load8_u offset=12
        i32.store8 offset=28
        local.get 3
        local.get 3
        i32.load offset=8
        i32.store offset=24
        i32.const 1
        local.get 3
        i32.const 24
        i32.add
        local.tee 0
        call 104
        local.get 0
        local.get 2
        call 105
        local.get 4
        local.get 3
        i32.load offset=24
        local.get 3
        i32.load8_u offset=28
        call 106
        br 1 (;@1;)
      end
      local.get 3
      i32.const 16
      i32.add
      call 103
      local.get 3
      local.get 3
      i32.load8_u offset=20
      i32.store8 offset=28
      local.get 3
      local.get 3
      i32.load offset=16
      i32.store offset=24
      i32.const 2
      local.get 3
      i32.const 24
      i32.add
      local.tee 0
      call 104
      local.get 0
      local.get 2
      call 105
      local.get 4
      local.get 3
      i32.load offset=24
      local.get 3
      i32.load8_u offset=28
      call 106
    end
    local.get 3
    i32.const 32
    i32.add
    global.set 0)
  (func (;57;) (type 5) (param i32 i32 i32)
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    call 110
    local.tee 0
    local.get 1
    local.get 2
    call 31
    drop
    local.get 0
    call 30
    unreachable)
  (func (;58;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    i32.const 1049152
    i32.const 23
    call 110
    local.tee 4
    local.get 0
    local.get 1
    call 31
    drop
    local.get 4
    i32.const 1049175
    i32.const 3
    call 31
    drop
    local.get 4
    local.get 2
    local.get 3
    call 31
    drop
    local.get 4
    call 30
    unreachable)
  (func (;59;) (type 1)
    (local i32)
    call 0
    i32.const 1
    call 39
    i32.const 0
    call 54
    local.tee 0
    local.get 0
    call 60
    call 61)
  (func (;60;) (type 3) (param i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 16
    i32.add
    local.tee 2
    i32.const 24
    i32.add
    i64.const 0
    i64.store
    local.get 2
    i32.const 16
    i32.add
    i64.const 0
    i64.store
    local.get 2
    i32.const 8
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i64.const 0
    i64.store offset=16
    local.get 0
    call 7
    local.tee 2
    local.set 3
    local.get 2
    i32.const 33
    i32.ge_u
    if  ;; label = @1
      i32.const 1049286
      i32.const 28
      call 9
      unreachable
    end
    local.get 1
    i32.const 8
    i32.add
    local.get 1
    i32.const 16
    i32.add
    local.get 3
    call 50
    local.get 0
    i32.const 0
    local.get 1
    i32.load offset=8
    local.tee 2
    local.get 1
    i32.load offset=12
    local.tee 3
    call 96
    drop
    local.get 1
    i32.const 48
    i32.add
    local.tee 0
    i32.const 24
    i32.add
    i64.const 0
    i64.store
    local.get 0
    i32.const 16
    i32.add
    i64.const 0
    i64.store
    local.get 0
    i32.const 8
    i32.add
    i64.const 0
    i64.store
    local.get 1
    i64.const 0
    i64.store offset=48
    local.get 2
    local.get 3
    local.get 0
    call 23
    drop
    local.get 0
    call 97
    local.get 1
    i32.const 80
    i32.add
    global.set 0)
  (func (;61;) (type 0) (param i32 i32)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    i32.const 1048818
    i32.const 8
    call 3
    local.set 3
    i32.const 1049197
    i32.const 5
    call 3
    local.tee 4
    local.get 3
    call 20
    drop
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            local.get 4
            call 43
            local.tee 4
            call 7
            local.tee 3
            i32.const 8
            i32.gt_u
            br_if 0 (;@4;)
            local.get 2
            i64.const 0
            i64.store offset=24
            local.get 4
            i32.const 0
            local.get 2
            local.get 3
            i32.sub
            i32.const 32
            i32.add
            local.get 3
            call 51
            br_if 0 (;@4;)
            local.get 2
            i64.load offset=24
            local.tee 5
            i64.const 56
            i64.shl
            local.get 5
            i64.const 40
            i64.shl
            i64.const 71776119061217280
            i64.and
            i64.or
            local.get 5
            i64.const 24
            i64.shl
            i64.const 280375465082880
            i64.and
            local.get 5
            i64.const 8
            i64.shl
            i64.const 1095216660480
            i64.and
            i64.or
            i64.or
            local.get 5
            i64.const 8
            i64.shr_u
            i64.const 4278190080
            i64.and
            local.get 5
            i64.const 24
            i64.shr_u
            i64.const 16711680
            i64.and
            i64.or
            local.get 5
            i64.const 40
            i64.shr_u
            i64.const 65280
            i64.and
            local.get 5
            i64.const 56
            i64.shr_u
            i64.or
            i64.or
            i64.or
            local.tee 5
            i64.const 256
            i64.ge_u
            br_if 1 (;@3;)
            local.get 5
            i64.const 254
            i64.and
            i64.eqz
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            call 91
            local.get 1
            call 84
            local.get 2
            i32.const 8
            i32.add
            local.get 1
            call 55
            local.get 2
            i32.load offset=8
            i32.eqz
            br_if 3 (;@1;)
            i32.const 1048826
            i32.const 18
            call 78
            unreachable
          end
          i32.const 1049042
          i32.const 14
          call 93
          unreachable
        end
        i32.const 1049042
        i32.const 14
        call 93
        unreachable
      end
      local.get 2
      i32.const 16
      i32.add
      call 94
      local.get 2
      local.get 2
      i32.load8_u offset=20
      i32.store8 offset=28
      local.get 2
      local.get 2
      i32.load offset=16
      i32.store offset=24
      i32.const 1048818
      i32.const 8
      call 5
      local.tee 0
      call 98
      local.get 2
      i32.const 24
      i32.add
      local.tee 1
      local.get 0
      call 105
      local.get 1
      i32.const 1049178
      i32.const 19
      call 107
      local.get 2
      i32.load offset=24
      local.get 2
      i32.load8_u offset=28
      call 95
      call 30
      unreachable
    end
    local.get 2
    i32.const 32
    i32.add
    global.set 0)
  (func (;62;) (type 1)
    (local i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 0
    global.set 0
    call 0
    i32.const 1
    call 39
    local.get 0
    i32.const 16
    i32.add
    i32.const 0
    call 54
    call 60
    local.tee 1
    call 63
    local.get 0
    i32.load8_u offset=47
    call 64
    i32.const 255
    i32.and
    i32.eq
    if (result i32)  ;; label = @1
      local.get 0
      i32.const 8
      i32.add
      local.get 1
      call 55
      local.get 0
      i32.load offset=12
      local.set 2
      local.get 0
      i32.load offset=8
      i32.const 1
      i32.ne
    else
      i32.const 1
    end
    local.get 2
    call 65
    local.get 0
    i32.const 48
    i32.add
    global.set 0)
  (func (;63;) (type 0) (param i32 i32)
    local.get 0
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 24
    i32.add
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 16
    i32.add
    i64.const 0
    i64.store align=1
    local.get 0
    i32.const 8
    i32.add
    i64.const 0
    i64.store align=1
    local.get 1
    i32.const 0
    local.get 0
    i32.const 32
    call 96
    drop)
  (func (;64;) (type 6) (result i32)
    (local i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    call 69
    call 63
    local.get 0
    i32.load8_u offset=31
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func (;65;) (type 0) (param i32 i32)
    local.get 0
    i32.eqz
    if  ;; label = @1
      local.get 1
      call 14
      drop
    end)
  (func (;66;) (type 1)
    (local i32 i32 i32 i32 i32)
    call 0
    call 67
    i32.const 0
    call 39
    call 68
    call 5
    local.tee 1
    call 44
    local.set 3
    call 69
    i64.const 0
    call 1
    local.set 0
    i32.const 1059408
    call 10
    drop
    block  ;; label = @1
      local.get 3
      i32.eqz
      if  ;; label = @2
        local.get 1
        i32.const 1059440
        call 10
        drop
        i32.const 1059408
        i32.const 1059440
        local.get 1
        call 7
        i64.const 0
        local.get 0
        call 11
        br 1 (;@1;)
      end
      i32.const 1059408
      local.get 0
      call 12
    end
    local.get 0
    i64.const 0
    i32.const 1048884
    i32.const 9
    call 3
    call 5
    call 13
    drop)
  (func (;67;) (type 1)
    call 71
    call 68
    call 29
    i32.const 1
    i32.ge_s
    if  ;; label = @1
      return
    end
    i32.const 1049074
    i32.const 36
    call 9
    unreachable)
  (func (;68;) (type 6) (result i32)
    (local i32)
    call 5
    local.tee 0
    call 27
    local.get 0)
  (func (;69;) (type 6) (result i32)
    (local i32)
    call 5
    local.tee 0
    call 22
    local.get 0)
  (func (;70;) (type 1)
    call 0
    i32.const 0
    call 39
    call 71
    call 14
    drop)
  (func (;71;) (type 6) (result i32)
    (local i32)
    call 5
    local.tee 0
    call 28
    local.get 0)
  (func (;72;) (type 1)
    call 0
    i32.const 0
    call 39
    call 64
    i64.extend_i32_u
    i64.const 255
    i64.and
    call 15)
  (func (;73;) (type 1)
    call 0
    i32.const 0
    call 39
    call 74
    call 16)
  (func (;74;) (type 6) (result i32)
    (local i32)
    i32.const 1048898
    i32.const 17
    call 3
    call 43
    i64.const 0
    call 1
    local.tee 0
    call 24
    drop
    local.get 0)
  (func (;75;) (type 1)
    call 0
    i32.const 1
    call 39
    i32.const 0
    call 54
    call 60
    call 14
    drop)
  (func (;76;) (type 1)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 0
    global.set 0
    call 0
    i32.const 1
    call 39
    local.get 0
    i32.const 0
    call 54
    call 60
    call 63
    local.get 0
    i64.load8_u offset=31
    call 15
    local.get 0
    i32.const 32
    i32.add
    global.set 0)
  (func (;77;) (type 1)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 80
    i32.sub
    local.tee 0
    global.set 0
    block  ;; label = @1
      call 17
      i32.eqz
      if  ;; label = @2
        i64.const 0
        call 1
        local.tee 1
        call 18
        i32.const 1
        call 39
        i32.const 0
        call 54
        local.tee 3
        call 60
        local.set 2
        local.get 3
        local.get 2
        call 61
        local.get 1
        call 74
        call 19
        i32.eqz
        br_if 1 (;@1;)
        i32.const 1048844
        i32.const 40
        call 78
        unreachable
      end
      i32.const 1048727
      i32.const 37
      call 9
      unreachable
    end
    local.get 2
    i32.const 1
    call 68
    local.tee 4
    call 45
    call 56
    i32.const 1049329
    call 97
    drop
    local.get 0
    i32.const 48
    i32.add
    local.tee 1
    local.get 4
    local.get 3
    call 79
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    call 80
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    call 81
    local.get 0
    i32.const 68
    i32.add
    local.get 0
    i64.load offset=8
    i64.store align=4
    local.get 0
    i32.const 1
    i32.store offset=64
    local.get 0
    local.get 0
    i64.load offset=16
    i64.store offset=48
    local.get 0
    local.get 0
    i64.load offset=24
    i64.store offset=56
    local.get 1
    call 82
    unreachable)
  (func (;78;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 9
    unreachable)
  (func (;79;) (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    call 5
    local.set 4
    i32.const 1048716
    i32.const 11
    call 3
    local.set 5
    call 5
    local.set 3
    i64.const 0
    call 1
    local.set 6
    local.get 0
    local.get 5
    i32.store offset=20
    local.get 0
    local.get 4
    i32.store offset=16
    local.get 0
    local.get 6
    i32.store offset=12
    local.get 0
    local.get 1
    i32.store offset=8
    local.get 0
    local.get 3
    i32.store offset=24
    local.get 0
    i64.const -1
    i64.store
    call 5
    drop
    local.get 3
    local.get 2
    call 45
    call 49)
  (func (;80;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i64 i64)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 4
    global.set 0
    local.get 1
    i32.load offset=28
    local.set 12
    local.get 1
    i32.load offset=24
    local.set 9
    local.get 1
    i32.load offset=20
    local.set 7
    local.get 1
    i32.load offset=12
    local.set 5
    local.get 1
    i32.load offset=8
    local.set 3
    local.get 1
    i64.load
    local.set 16
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 1
          i32.load offset=16
          local.tee 6
          call 120
          br_table 2 (;@1;) 1 (;@2;) 0 (;@3;)
        end
        call 5
        call 5
        local.set 2
        call 5
        drop
        local.get 2
        local.get 3
        call 45
        call 49
        local.get 6
        call 120
        local.set 3
        call 5
        local.tee 5
        local.get 3
        i64.extend_i32_u
        call 121
        local.get 2
        local.get 5
        call 49
        local.get 6
        call 7
        local.set 11
        local.get 4
        i32.const 16
        i32.add
        local.set 13
        i32.const 0
        local.set 3
        loop  ;; label = @3
          local.get 3
          i32.const 16
          i32.add
          local.tee 5
          local.get 11
          i32.le_u
          if  ;; label = @4
            local.get 13
            i64.const 0
            i64.store
            local.get 4
            i64.const 0
            i64.store offset=8
            local.get 6
            local.get 3
            local.get 4
            i32.const 8
            i32.add
            local.tee 3
            i32.const 16
            call 51
            drop
            local.get 4
            i32.const 0
            i32.store offset=28
            local.get 3
            local.get 4
            i32.const 28
            i32.add
            local.tee 8
            call 122
            local.get 3
            local.get 8
            call 123
            local.set 15
            local.get 3
            local.get 8
            call 122
            local.set 8
            local.get 4
            call 5
            i32.store offset=8
            local.get 3
            call 117
            local.get 2
            local.get 4
            i32.load offset=8
            call 49
            call 5
            local.tee 3
            local.get 15
            call 121
            local.get 2
            local.get 3
            call 49
            call 5
            drop
            local.get 2
            local.get 8
            call 40
            call 49
            local.get 5
            local.set 3
            br 1 (;@3;)
          end
        end
        call 5
        drop
        local.get 2
        local.get 7
        call 45
        call 49
        call 69
        local.set 3
        i64.const 0
        call 1
        local.set 5
        i32.const 1049266
        i32.const 20
        call 3
        local.set 7
        local.get 2
        local.get 9
        call 114
        local.set 9
        local.set 6
        br 1 (;@1;)
      end
      local.get 4
      i32.const 16
      i32.add
      i64.const 0
      i64.store
      local.get 4
      i64.const 0
      i64.store offset=8
      local.get 6
      i32.const 0
      local.get 4
      i32.const 8
      i32.add
      local.tee 2
      i32.const 16
      call 51
      local.get 4
      i32.const 0
      i32.store offset=28
      local.get 2
      local.get 4
      i32.const 28
      i32.add
      local.tee 8
      call 122
      local.set 10
      local.get 2
      local.get 8
      call 123
      local.set 15
      local.get 2
      local.get 8
      call 122
      local.set 2
      br_if 0 (;@1;)
      local.get 2
      local.set 5
      local.get 10
      call 44
      i32.eqz
      if  ;; label = @2
        call 5
        local.set 6
        call 5
        local.tee 2
        local.get 10
        call 116
        block (result i32)  ;; label = @3
          local.get 15
          i64.eqz
          if  ;; label = @4
            local.get 2
            local.get 5
            call 115
            local.get 2
            local.get 7
            call 118
            i64.const 0
            call 1
            local.set 5
            i32.const 1049254
            i32.const 12
            call 3
            br 1 (;@3;)
          end
          call 5
          local.tee 10
          local.get 15
          call 121
          local.get 2
          local.get 10
          call 49
          local.get 2
          local.get 5
          call 115
          call 5
          drop
          local.get 2
          local.get 3
          call 45
          call 49
          local.get 2
          local.get 7
          call 118
          call 69
          local.set 3
          i64.const 0
          call 1
          local.set 5
          i32.const 1049239
          i32.const 15
          call 3
        end
        local.set 7
        local.get 2
        local.get 9
        call 114
        local.set 9
        br 1 (;@1;)
      end
      local.get 6
      i32.const 1049384
      i32.const 0
      call 33
      drop
    end
    local.get 1
    local.get 12
    i32.store offset=28
    local.get 1
    local.get 9
    i32.store offset=24
    local.get 1
    local.get 7
    i32.store offset=20
    local.get 1
    local.get 6
    i32.store offset=16
    local.get 1
    local.get 5
    i32.store offset=12
    local.get 1
    local.get 3
    i32.store offset=8
    local.get 1
    local.get 16
    i64.store
    local.get 0
    i32.const 0
    i32.store offset=16
    local.get 0
    local.get 9
    i32.store offset=12
    local.get 0
    local.get 7
    i32.store offset=8
    local.get 0
    local.get 5
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 4
    i32.const 32
    i32.add
    global.set 0)
  (func (;81;) (type 0) (param i32 i32)
    (local i32 i32)
    i32.const 1048935
    i32.const 22
    call 3
    local.set 3
    call 5
    local.set 2
    call 5
    drop
    local.get 2
    local.get 1
    call 45
    call 49
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 3
    i32.store)
  (func (;82;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32 i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load offset=16
    i32.const 1
    i32.eq
    if  ;; label = @1
      call 42
      local.get 1
      i32.const 8
      i32.add
      call 103
      local.get 1
      local.get 1
      i32.load8_u offset=12
      i32.store8 offset=20
      local.get 1
      local.get 1
      i32.load offset=8
      i32.store offset=16
      local.get 0
      i32.const 20
      i32.add
      i32.load
      local.get 1
      i32.const 16
      i32.add
      local.tee 3
      call 119
      local.get 1
      local.get 0
      i32.const 24
      i32.add
      i32.load
      local.tee 4
      call 52
      local.tee 2
      i32.const 24
      i32.shl
      local.get 2
      i32.const 8
      i32.shl
      i32.const 16711680
      i32.and
      i32.or
      local.get 2
      i32.const 8
      i32.shr_u
      i32.const 65280
      i32.and
      local.get 2
      i32.const 24
      i32.shr_u
      i32.or
      i32.or
      i32.store offset=28
      local.get 3
      local.get 1
      i32.const 28
      i32.add
      i32.const 4
      call 107
      local.get 4
      call 7
      local.set 6
      i32.const 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        i32.const 4
        i32.add
        local.tee 3
        local.get 6
        i32.le_u
        if  ;; label = @3
          local.get 1
          i32.const 0
          i32.store offset=28
          local.get 4
          local.get 2
          local.get 1
          i32.const 28
          i32.add
          i32.const 4
          call 96
          drop
          local.get 1
          i32.load offset=28
          local.tee 2
          i32.const 24
          i32.shl
          local.get 2
          i32.const 8
          i32.shl
          i32.const 16711680
          i32.and
          i32.or
          local.get 2
          i32.const 8
          i32.shr_u
          i32.const 65280
          i32.and
          local.get 2
          i32.const 24
          i32.shr_u
          i32.or
          i32.or
          local.get 1
          i32.const 16
          i32.add
          call 119
          local.get 3
          local.set 2
          br 1 (;@2;)
        end
      end
      local.get 1
      i32.load offset=16
      local.get 1
      i32.load8_u offset=20
      call 106
    end
    local.get 0
    i32.load
    local.get 0
    i32.load offset=4
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 26
    unreachable)
  (func (;83;) (type 1)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 0
    call 67
    i32.const 1
    call 39
    i32.const 0
    call 54
    call 60
    local.tee 1
    call 84
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    call 55
    local.get 0
    i32.load offset=8
    i32.const 1
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.const 0
      local.get 0
      call 56
    end
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;84;) (type 2) (param i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    local.get 0
    call 63
    local.get 1
    i32.load8_u offset=31
    call 64
    i32.const 255
    i32.and
    i32.eq
    if  ;; label = @1
      local.get 1
      i32.const 32
      i32.add
      global.set 0
      return
    end
    i32.const 1048789
    i32.const 29
    call 78
    unreachable)
  (func (;85;) (type 1)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 0
    i32.const 1
    call 39
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    call 54
    call 60
    call 86
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 65
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;86;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 16
    i32.add
    local.get 1
    call 63
    block (result i32)  ;; label = @1
      i32.const 1
      local.get 2
      i32.load8_u offset=47
      call 64
      i32.const 255
      i32.and
      i32.ne
      br_if 0 (;@1;)
      drop
      local.get 2
      i32.const 8
      i32.add
      local.get 1
      call 55
      local.get 2
      i32.load offset=12
      local.set 3
      local.get 2
      i32.load offset=8
      i32.const 2
      i32.ne
    end
    local.set 1
    local.get 0
    local.get 3
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store
    local.get 2
    i32.const 48
    i32.add
    global.set 0)
  (func (;87;) (type 1)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 0
    global.set 0
    call 0
    i32.const 1
    call 39
    i32.const 0
    call 54
    local.tee 1
    call 7
    i32.const 32
    i32.ne
    if  ;; label = @1
      i32.const 1048926
      i32.const 9
      i32.const 1049361
      i32.const 16
      call 58
      unreachable
    end
    local.get 0
    i32.const 8
    i32.add
    local.get 1
    call 86
    local.get 0
    i32.load offset=8
    local.get 0
    i32.load offset=12
    call 65
    local.get 0
    i32.const 16
    i32.add
    global.set 0)
  (func (;88;) (type 1)
    (local i32 i32 i32 i64)
    call 0
    call 67
    i32.const 2
    local.set 0
    i32.const 2
    call 39
    i32.const 0
    call 54
    local.set 1
    block  ;; label = @1
      block  ;; label = @2
        i32.const 1
        call 8
        local.tee 3
        i64.const 1
        i64.le_u
        if  ;; label = @3
          local.get 3
          i32.wrap_i64
          i32.const 1
          i32.sub
          br_if 2 (;@1;)
          br 1 (;@2;)
        end
        i32.const 1049202
        i32.const 5
        i32.const 1048698
        i32.const 18
        call 58
        unreachable
      end
      i32.const 1
      local.set 0
    end
    i32.const 1049197
    i32.const 5
    call 3
    local.tee 2
    local.get 1
    call 20
    drop
    local.get 0
    local.get 2
    call 89)
  (func (;89;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i64.const 0
    i64.store offset=8
    local.get 0
    i64.extend_i32_u
    i64.const 255
    i64.and
    local.set 6
    i32.const 56
    local.set 0
    i32.const 1
    local.set 4
    block  ;; label = @1
      loop  ;; label = @2
        local.get 0
        i32.const 0
        i32.ge_s
        if  ;; label = @3
          local.get 6
          local.get 0
          i32.const 56
          i32.and
          i64.extend_i32_u
          i64.shr_u
          i32.wrap_i64
          local.set 5
          block (result i32)  ;; label = @4
            block  ;; label = @5
              local.get 4
              i32.eqz
              if  ;; label = @6
                local.get 3
                i32.const 8
                i32.lt_u
                br_if 1 (;@5;)
                br 5 (;@1;)
              end
              i32.const 1
              local.get 5
              i32.eqz
              br_if 1 (;@4;)
              drop
              local.get 3
              i32.const 8
              i32.ge_u
              br_if 4 (;@1;)
            end
            local.get 2
            i32.const 8
            i32.add
            local.get 3
            i32.add
            local.get 5
            i32.store8
            local.get 3
            i32.const 1
            i32.add
            local.set 3
            i32.const 0
          end
          local.set 4
          local.get 0
          i32.const -8
          i32.add
          local.set 0
          br 1 (;@2;)
        end
      end
      local.get 2
      i32.const 0
      local.get 3
      local.get 2
      i32.const 8
      i32.add
      i32.const 8
      call 108
      local.get 1
      local.get 2
      i32.load
      local.get 2
      i32.load offset=4
      call 3
      call 4
      drop
      local.get 2
      i32.const 16
      i32.add
      global.set 0
      return
    end
    call 109
    unreachable)
  (func (;90;) (type 1)
    call 0
    i32.const 1
    call 39
    i32.const 0
    call 54
    call 91)
  (func (;91;) (type 2) (param i32)
    (local i32 i32 i32 i32 i32)
    global.get 0
    i32.const 48
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 0
          call 7
          local.tee 4
          i32.const 8
          i32.lt_u
          if  ;; label = @4
            i32.const 1048576
            local.set 1
            i32.const 28
            local.set 2
            br 1 (;@3;)
          end
          local.get 4
          i32.const 32
          i32.gt_u
          if  ;; label = @4
            i32.const 1048604
            local.set 1
            i32.const 13
            local.set 2
            br 1 (;@3;)
          end
          i32.const 24
          local.set 2
          local.get 3
          i32.const 16
          i32.add
          local.tee 1
          i32.const 24
          i32.add
          i64.const 0
          i64.store
          local.get 3
          i32.const 32
          i32.add
          i64.const 0
          i64.store
          local.get 3
          i32.const 24
          i32.add
          i64.const 0
          i64.store
          local.get 3
          i64.const 0
          i64.store offset=16
          local.get 3
          i32.const 8
          i32.add
          local.get 1
          local.get 4
          call 50
          local.get 0
          i32.const 0
          local.get 3
          i32.load offset=8
          local.tee 4
          local.get 3
          i32.load offset=12
          local.tee 5
          call 96
          if  ;; label = @4
            i32.const 1048617
            local.set 1
            br 1 (;@3;)
          end
          local.get 5
          local.get 0
          call 7
          i32.const -7
          i32.add
          local.tee 0
          i32.lt_u
          br_if 2 (;@1;)
          i32.const 1048648
          local.set 1
          i32.const 12
          local.set 2
          local.get 5
          local.get 0
          i32.sub
          i32.const 7
          i32.ne
          br_if 0 (;@3;)
          local.get 0
          local.get 4
          i32.add
          i32.const 1048641
          i32.const 7
          call 128
          br_if 0 (;@3;)
          local.get 0
          i32.const 3
          i32.lt_u
          if  ;; label = @4
            i32.const 1048660
            local.set 1
            i32.const 17
            local.set 2
            br 1 (;@3;)
          end
          loop  ;; label = @4
            local.get 0
            i32.eqz
            br_if 2 (;@2;)
            local.get 0
            i32.const -1
            i32.add
            local.set 0
            local.get 4
            i32.load8_u
            local.set 2
            local.get 4
            i32.const 1
            i32.add
            local.tee 1
            local.set 4
            local.get 2
            i32.const -97
            i32.add
            i32.const 255
            i32.and
            i32.const 26
            i32.lt_u
            br_if 0 (;@4;)
            local.get 1
            local.set 4
            local.get 2
            i32.const -48
            i32.add
            i32.const 255
            i32.and
            i32.const 10
            i32.lt_u
            br_if 0 (;@4;)
          end
          i32.const 1048677
          local.set 1
          i32.const 21
          local.set 2
        end
        local.get 1
        local.get 2
        call 78
        unreachable
      end
      local.get 3
      i32.const 48
      i32.add
      global.set 0
      return
    end
    call 127
    unreachable)
  (func (;92;) (type 1)
    call 0
    i32.const 0
    call 39
    i32.const 1048893
    i32.const 5
    call 21)
  (func (;93;) (type 0) (param i32 i32)
    (local i32)
    i32.const 1049207
    i32.const 22
    call 110
    local.tee 2
    local.get 0
    local.get 1
    call 31
    drop
    local.get 2
    call 30
    unreachable)
  (func (;94;) (type 2) (param i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    block (result i32)  ;; label = @1
      i32.const 1059404
      i32.load8_u
      i32.eqz
      if  ;; label = @2
        i32.const 1
        local.set 2
        i32.const 1059404
        i32.const 1
        i32.store8
        i32.const 1049400
        i32.const 0
        i32.store
        local.get 1
        i32.const 8
        i32.add
        i32.const 0
        call 113
        local.get 1
        i32.load offset=8
        local.get 1
        i32.load offset=12
        i32.const 1049384
        i32.const 0
        call 124
        call 5
        br 1 (;@1;)
      end
      i32.const 1049384
      i32.const 0
      call 3
    end
    local.set 3
    local.get 0
    local.get 2
    i32.store8 offset=4
    local.get 0
    local.get 3
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;95;) (type 4) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 1
    i32.store8 offset=12
    local.get 2
    local.get 0
    i32.store offset=8
    local.get 2
    i32.const 8
    i32.add
    call 125
    local.get 2
    i32.load offset=8
    local.get 2
    i32.load8_u offset=12
    if  ;; label = @1
      i32.const 1049400
      i32.const 0
      i32.store
      i32.const 1059404
      i32.const 0
      i32.store8
    end
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;96;) (type 9) (param i32 i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    local.get 3
    call 51)
  (func (;97;) (type 3) (param i32) (result i32)
    local.get 0
    i32.const 32
    call 3)
  (func (;98;) (type 5) (param i32 i32 i32)
    local.get 2
    local.get 0
    local.get 1
    call 99)
  (func (;99;) (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call 33
    drop)
  (func (;100;) (type 0) (param i32 i32)
    local.get 0
    local.get 1
    call 20
    drop)
  (func (;101;) (type 9) (param i32 i32 i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    block (result i32)  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=8
        i32.eqz
        if  ;; label = @3
          local.get 0
          i32.load
          local.tee 6
          call 7
          local.set 5
          i32.const 1059404
          i32.load8_u
          br_if 1 (;@2;)
          local.get 5
          i32.const 10000
          i32.gt_u
          br_if 1 (;@2;)
          i32.const 1049400
          local.get 5
          i32.store
          i32.const 1059404
          i32.const 1
          i32.store8
          local.get 4
          i32.const 8
          i32.add
          local.get 5
          call 113
          local.get 6
          i32.const 0
          local.get 4
          i32.load offset=8
          local.get 4
          i32.load offset=12
          call 96
          drop
          local.get 0
          i32.const 1
          i32.store8 offset=8
        end
        i32.const 1
        local.get 1
        local.get 3
        i32.add
        local.tee 5
        i32.const 1049400
        i32.load
        i32.gt_u
        br_if 1 (;@1;)
        drop
        local.get 4
        local.get 1
        local.get 5
        call 126
        local.get 2
        local.get 3
        local.get 4
        i32.load
        local.get 4
        i32.load offset=4
        call 124
        i32.const 0
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      i32.store8 offset=8
      local.get 6
      local.get 1
      local.get 2
      local.get 3
      call 96
    end
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;102;) (type 7) (param i32 i32 i32 i32)
    local.get 1
    local.get 2
    local.get 3
    call 5
    local.tee 1
    call 36
    local.set 2
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 0
    local.get 2
    i32.eqz
    i32.store)
  (func (;103;) (type 2) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 1
    i32.const 8
    i32.add
    call 94
    local.get 1
    i32.load offset=8
    local.set 2
    local.get 0
    local.get 1
    i32.load8_u offset=12
    i32.const 1
    i32.and
    i32.store8 offset=4
    local.get 0
    local.get 2
    i32.store
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;104;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    i32.store8 offset=15
    local.get 1
    local.get 2
    i32.const 15
    i32.add
    i32.const 1
    call 107
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;105;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.load8_u offset=4
    local.set 2
    local.get 0
    i32.const 0
    i32.store8 offset=4
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          local.get 2
          i32.const 1
          i32.and
          local.tee 4
          if  ;; label = @4
            local.get 1
            call 7
            local.set 2
            i32.const 10000
            i32.const 1049400
            i32.load
            local.tee 5
            i32.sub
            local.get 2
            i32.lt_u
            br_if 2 (;@2;)
            local.get 3
            i32.const 8
            i32.add
            local.get 5
            local.get 2
            local.get 5
            i32.add
            local.tee 2
            call 112
            local.get 1
            i32.const 0
            local.get 3
            i32.load offset=8
            local.get 3
            i32.load offset=12
            call 96
            drop
            i32.const 1049400
            local.get 2
            i32.store
            br 1 (;@3;)
          end
          local.get 0
          i32.load
          local.get 1
          call 100
        end
        local.get 0
        local.get 4
        i32.store8 offset=4
        br 1 (;@1;)
      end
      local.get 0
      call 125
      local.get 0
      i32.load
      local.get 1
      call 100
      local.get 0
      i32.load8_u offset=4
      local.get 0
      local.get 4
      i32.store8 offset=4
      i32.const 1
      i32.and
      i32.eqz
      br_if 0 (;@1;)
      i32.const 1049400
      i32.const 0
      i32.store
      i32.const 1059404
      i32.const 0
      i32.store8
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;106;) (type 5) (param i32 i32 i32)
    local.get 0
    local.get 1
    local.get 2
    call 95
    call 4
    drop)
  (func (;107;) (type 5) (param i32 i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    block  ;; label = @1
      block  ;; label = @2
        local.get 0
        i32.load8_u offset=4
        if  ;; label = @3
          i32.const 10000
          i32.const 1049400
          i32.load
          local.tee 4
          i32.sub
          local.get 2
          i32.lt_u
          br_if 1 (;@2;)
          local.get 3
          i32.const 8
          i32.add
          local.get 4
          local.get 2
          local.get 4
          i32.add
          local.tee 0
          call 112
          local.get 3
          i32.load offset=8
          local.get 3
          i32.load offset=12
          local.get 1
          local.get 2
          call 124
          i32.const 1049400
          local.get 0
          i32.store
          br 2 (;@1;)
        end
        local.get 0
        i32.load
        local.get 1
        local.get 2
        call 31
        drop
        br 1 (;@1;)
      end
      local.get 0
      call 125
      local.get 0
      i32.load
      local.get 1
      local.get 2
      call 31
      drop
    end
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;108;) (type 10) (param i32 i32 i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.ge_u
      if  ;; label = @2
        local.get 2
        local.get 4
        i32.le_u
        br_if 1 (;@1;)
        call 109
        unreachable
      end
      call 109
      unreachable
    end
    local.get 0
    local.get 2
    local.get 1
    i32.sub
    i32.store offset=4
    local.get 0
    local.get 1
    local.get 3
    i32.add
    i32.store)
  (func (;109;) (type 1)
    call 127
    unreachable)
  (func (;110;) (type 4) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 3)
  (func (;111;) (type 7) (param i32 i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 4
    global.set 0
    local.get 4
    i32.const 8
    i32.add
    i32.const 0
    local.get 3
    local.get 1
    local.get 2
    call 108
    local.get 4
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 4
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 4
    i32.const 16
    i32.add
    global.set 0)
  (func (;112;) (type 5) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 8
    i32.add
    local.get 1
    local.get 2
    i32.const 1049404
    i32.const 10000
    call 108
    local.get 3
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 3
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 3
    i32.const 16
    i32.add
    global.set 0)
  (func (;113;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 8
    i32.add
    i32.const 1049404
    i32.const 10000
    local.get 1
    call 111
    local.get 2
    i32.load offset=12
    local.set 1
    local.get 0
    local.get 2
    i32.load offset=8
    i32.store
    local.get 0
    local.get 1
    i32.store offset=4
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;114;) (type 4) (param i32 i32) (result i32)
    local.get 0
    local.get 1
    call 20
    drop
    local.get 0)
  (func (;115;) (type 0) (param i32 i32)
    call 5
    drop
    local.get 0
    local.get 1
    call 40
    call 49)
  (func (;116;) (type 0) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    call 5
    i32.store offset=12
    local.get 1
    local.get 2
    i32.const 12
    i32.add
    call 117
    local.get 0
    local.get 2
    i32.load offset=12
    call 49
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;117;) (type 0) (param i32 i32)
    local.get 0
    call 44
    i32.eqz
    if  ;; label = @1
      local.get 1
      local.get 0
      call 45
      i32.store
      return
    end
    i32.const 1049377
    i32.const 4
    local.get 1
    i32.load
    call 98)
  (func (;118;) (type 0) (param i32 i32)
    call 5
    drop
    local.get 0
    local.get 1
    call 45
    call 49)
  (func (;119;) (type 0) (param i32 i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    local.get 0
    call 7
    local.tee 3
    i32.const 24
    i32.shl
    local.get 3
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    i32.or
    local.get 3
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 3
    i32.const 24
    i32.shr_u
    i32.or
    i32.or
    i32.store offset=12
    local.get 1
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    call 107
    local.get 1
    local.get 0
    call 105
    local.get 2
    i32.const 16
    i32.add
    global.set 0)
  (func (;120;) (type 3) (param i32) (result i32)
    local.get 0
    call 7
    i32.const 4
    i32.shr_u)
  (func (;121;) (type 12) (param i32 i64)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 3
    global.set 0
    local.get 0
    i32.const 1049384
    i32.const 0
    call 99
    local.get 3
    local.get 1
    i64.const 40
    i64.shl
    i64.const 71776119061217280
    i64.and
    local.get 1
    i64.const 56
    i64.shl
    i64.or
    local.get 1
    i64.const 24
    i64.shl
    i64.const 280375465082880
    i64.and
    local.get 1
    i64.const 8
    i64.shl
    i64.const 1095216660480
    i64.and
    i64.or
    i64.or
    local.get 1
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 1
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 1
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 1
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or
    i64.store offset=8
    block  ;; label = @1
      local.get 1
      i64.eqz
      i32.eqz
      if  ;; label = @2
        loop  ;; label = @3
          local.get 2
          i32.const 8
          i32.eq
          br_if 2 (;@1;)
          local.get 3
          i32.const 8
          i32.add
          local.get 2
          i32.add
          local.tee 4
          i32.load8_u
          i32.eqz
          if  ;; label = @4
            local.get 2
            i32.const 1
            i32.add
            local.set 2
            br 1 (;@3;)
          end
        end
        local.get 0
        local.get 4
        i32.const 8
        local.get 2
        i32.sub
        call 31
        drop
      end
      local.get 3
      i32.const 16
      i32.add
      global.set 0
      return
    end
    call 109
    unreachable)
  (func (;122;) (type 4) (param i32 i32) (result i32)
    (local i32 i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 0
    i32.store offset=12
    local.get 1
    i32.load
    local.tee 4
    i32.const 4
    i32.add
    local.set 3
    local.get 2
    local.get 4
    local.get 3
    local.get 0
    i32.const 16
    call 108
    local.get 2
    i32.const 12
    i32.add
    i32.const 4
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 124
    local.get 1
    local.get 3
    i32.store
    local.get 2
    i32.load offset=12
    local.set 0
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 0
    i32.const 8
    i32.shl
    i32.const 16711680
    i32.and
    local.get 0
    i32.const 24
    i32.shl
    i32.or
    local.get 0
    i32.const 8
    i32.shr_u
    i32.const 65280
    i32.and
    local.get 0
    i32.const 24
    i32.shr_u
    i32.or
    i32.or)
  (func (;123;) (type 17) (param i32 i32) (result i64)
    (local i32 i32 i32 i64)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i64.const 0
    i64.store offset=8
    local.get 1
    i32.load
    local.tee 4
    i32.const 8
    i32.add
    local.set 3
    local.get 2
    local.get 4
    local.get 3
    local.get 0
    i32.const 16
    call 108
    local.get 2
    i32.const 8
    i32.add
    i32.const 8
    local.get 2
    i32.load
    local.get 2
    i32.load offset=4
    call 124
    local.get 1
    local.get 3
    i32.store
    local.get 2
    i64.load offset=8
    local.set 5
    local.get 2
    i32.const 16
    i32.add
    global.set 0
    local.get 5
    i64.const 40
    i64.shl
    i64.const 71776119061217280
    i64.and
    local.get 5
    i64.const 56
    i64.shl
    i64.or
    local.get 5
    i64.const 24
    i64.shl
    i64.const 280375465082880
    i64.and
    local.get 5
    i64.const 8
    i64.shl
    i64.const 1095216660480
    i64.and
    i64.or
    i64.or
    local.get 5
    i64.const 8
    i64.shr_u
    i64.const 4278190080
    i64.and
    local.get 5
    i64.const 24
    i64.shr_u
    i64.const 16711680
    i64.and
    i64.or
    local.get 5
    i64.const 40
    i64.shr_u
    i64.const 65280
    i64.and
    local.get 5
    i64.const 56
    i64.shr_u
    i64.or
    i64.or
    i64.or)
  (func (;124;) (type 7) (param i32 i32 i32 i32)
    local.get 1
    local.get 3
    i32.eq
    if  ;; label = @1
      local.get 0
      local.get 2
      local.get 1
      call 129
      return
    end
    call 109
    unreachable)
  (func (;125;) (type 2) (param i32)
    (local i32 i32)
    global.get 0
    i32.const 16
    i32.sub
    local.tee 1
    global.set 0
    local.get 0
    i32.load8_u offset=4
    local.get 0
    i32.const 0
    i32.store8 offset=4
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 1
      i32.const 8
      i32.add
      i32.const 0
      i32.const 1049400
      i32.load
      call 126
      local.get 0
      i32.load
      local.get 1
      i32.load offset=8
      local.get 1
      i32.load offset=12
      call 31
      drop
      i32.const 1049400
      i32.const 0
      i32.store
      i32.const 1059404
      i32.const 0
      i32.store8
    end
    local.get 1
    i32.const 16
    i32.add
    global.set 0)
  (func (;126;) (type 5) (param i32 i32 i32)
    block  ;; label = @1
      local.get 2
      local.get 1
      i32.ge_u
      if  ;; label = @2
        local.get 2
        i32.const 10000
        i32.le_u
        br_if 1 (;@1;)
        call 109
        unreachable
      end
      call 109
      unreachable
    end
    local.get 0
    local.get 2
    local.get 1
    i32.sub
    i32.store offset=4
    local.get 0
    local.get 1
    i32.const 1049404
    i32.add
    i32.store)
  (func (;127;) (type 1)
    i32.const 1049384
    i32.const 14
    call 9
    unreachable)
  (func (;128;) (type 8) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 130)
  (func (;129;) (type 5) (param i32 i32 i32)
    (local i32 i32 i32 i32 i32 i32)
    local.get 2
    i32.const 15
    i32.gt_u
    if  ;; label = @1
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 3
      local.get 0
      i32.add
      local.set 4
      local.get 3
      if  ;; label = @2
        local.get 1
        local.set 5
        loop  ;; label = @3
          local.get 0
          local.get 5
          i32.load8_u
          i32.store8
          local.get 5
          i32.const 1
          i32.add
          local.set 5
          local.get 0
          i32.const 1
          i32.add
          local.tee 0
          local.get 4
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      local.get 3
      i32.sub
      local.tee 2
      i32.const -4
      i32.and
      local.tee 6
      local.get 4
      i32.add
      local.set 0
      block  ;; label = @2
        local.get 1
        local.get 3
        i32.add
        local.tee 3
        i32.const 3
        i32.and
        if  ;; label = @3
          local.get 6
          i32.const 1
          i32.lt_s
          br_if 1 (;@2;)
          local.get 3
          i32.const 3
          i32.shl
          local.tee 1
          i32.const 24
          i32.and
          local.set 7
          i32.const 0
          local.get 1
          i32.sub
          i32.const 24
          i32.and
          local.set 8
          local.get 3
          i32.const -4
          i32.and
          local.tee 5
          i32.const 4
          i32.add
          local.set 1
          local.get 5
          i32.load
          local.set 5
          loop  ;; label = @4
            local.get 4
            local.get 5
            local.get 7
            i32.shr_u
            local.get 1
            i32.load
            local.tee 5
            local.get 8
            i32.shl
            i32.or
            i32.store
            local.get 1
            i32.const 4
            i32.add
            local.set 1
            local.get 4
            i32.const 4
            i32.add
            local.tee 4
            local.get 0
            i32.lt_u
            br_if 0 (;@4;)
          end
          br 1 (;@2;)
        end
        local.get 6
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 3
        local.set 1
        loop  ;; label = @3
          local.get 4
          local.get 1
          i32.load
          i32.store
          local.get 1
          i32.const 4
          i32.add
          local.set 1
          local.get 4
          i32.const 4
          i32.add
          local.tee 4
          local.get 0
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 2
      i32.const 3
      i32.and
      local.set 2
      local.get 3
      local.get 6
      i32.add
      local.set 1
    end
    local.get 2
    if  ;; label = @1
      local.get 0
      local.get 2
      i32.add
      local.set 2
      loop  ;; label = @2
        local.get 0
        local.get 1
        i32.load8_u
        i32.store8
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        local.get 0
        i32.const 1
        i32.add
        local.tee 0
        local.get 2
        i32.lt_u
        br_if 0 (;@2;)
      end
    end)
  (func (;130;) (type 8) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      loop  ;; label = @2
        local.get 0
        i32.load8_u
        local.tee 4
        local.get 1
        i32.load8_u
        local.tee 5
        i32.eq
        if  ;; label = @3
          local.get 0
          i32.const 1
          i32.add
          local.set 0
          local.get 1
          i32.const 1
          i32.add
          local.set 1
          local.get 2
          i32.const -1
          i32.add
          local.tee 2
          br_if 1 (;@2;)
          br 2 (;@1;)
        end
      end
      local.get 4
      local.get 5
      i32.sub
      local.set 3
    end
    local.get 3)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1059472))
  (global (;2;) i32 (i32.const 1059472))
  (export "memory" (memory 0))
  (export "init" (func 38))
  (export "callBack" (func 41))
  (export "canRegister" (func 59))
  (export "checkPending" (func 62))
  (export "claim" (func 66))
  (export "getContractOwner" (func 70))
  (export "getOwnShardId" (func 72))
  (export "getRegistrationCost" (func 73))
  (export "nameHash" (func 75))
  (export "nameShard" (func 76))
  (export "register" (func 77))
  (export "resetPending" (func 83))
  (export "resolve" (func 85))
  (export "resolveFromHash" (func 87))
  (export "setFeatureFlag" (func 88))
  (export "validateName" (func 90))
  (export "version" (func 92))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "name does not contain suffixname too longerror loading name bytes.numbatwrong suffixname is too shortcharacter not allowedinput out of rangeSetUserNamefunction does not accept DCDT paymentwrong number of argumentsname belongs to another shardregistername already takenshould pay exactly the registration costdns claim1.1.0registration_costvalue_statename_hashset_user_name_callbackcb_name_hashresultno callback function with that name exists in contractinvalid valueinput too longtoo many argumentsEndpoint can only be called by ownerserializer decode error: too few argumentsargument decode error ():  currently disabledfeat:valuestorage decode error: CB_CLOSUREDCDTNFTTransferDCDTTransferMultiDCDTNFTTransferfailed to load to byte arrayinput too short")
  (data (;1;) (i32.const 1049361) "bad array lengthREWA\00\00\00panic occurred"))
