typedef struct spgPickSplitIn
{
    int         nTuples;        /* number of leaf tuples */
    Datum      *datums;         /* their datums (array of length nTuples) */
    int         level;          /* current level (counting from zero) */
} spgPickSplitIn;

typedef struct spgPickSplitOut
{
    bool        hasPrefix;      /* new inner tuple should have a prefix? */
    Datum       prefixDatum;    /* if so, its value */

    int         nNodes;         /* number of nodes for new inner tuple */
    Datum      *nodeLabels;     /* their labels (or NULL for no labels) */

    int        *mapTuplesToNodes;   /* node index for each leaf tuple */
    Datum      *leafTupleDatums;    /* datum to store in each new leaf tuple */
} spgPickSplitOut;
