TupleTableSlot **
ExecForeignBatchInsert(EState *estate,
                  ResultRelInfo *rinfo,
                  TupleTableSlot **slots,
                  TupleTableSlot **planSlots,
                  int *numSlots);
