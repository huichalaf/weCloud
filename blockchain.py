import hashlib
import os, sys
from datetime import date, datetime, timedelta
from time import time as tm

class weCloudCoinBlock:
    def __init__(self):
        self.OriginalBlock = 'papo-send-3-leo'
        self.previousPapoBlock = ''
        self.listOfTransactions = ''
        self.blockData = ''
        self.blockHash = ''
        self.blockchain = ''

    def createNewBlock(self, previousBlockHash, transactionsList):
       
        if previousBlockHash != self.previousPapoBlock:
            sys.exit()

        self.previousPapoBlock = previousBlockHash
        
        date = datetime.now()
        date = str(date)
        
        transactionsList[-1] = transactionsList[-1]+date
        self.listOfTransactions = transactionsList
        
        self.blockData = '-'.join(self.listOfTransactions) + '-'+self.previousPapoBlock
        self.blockHash = hashlib.sha512(self.blockData.encode()).hexdigest()
    
    def addToChain(self):

        self.blockchain += '_'+self.blockHash
    
    def getPreviousBlock(self):
        return self.previousPapoBlock

    def writeBlockchain(self):
        with open('blockchain.txt', 'a') as f:
            f.write(self.blockchain)

blockchain = weCloudCoinBlock()
new = []
tiempo2, tiempo = 0, 0

while True:
    tiempo = tm()
    while tiempo2 <= 10:
        new.append(input("ingrese la nueva transaccion: "))
        tiempo2 = tm()-tiempo
    tiempo = 0
    tiempo2 = 0
    bloquePrevio = blockchain.getPreviousBlock()
    blockchain.createNewBlock(bloquePrevio, new)
    blockchain.addToChain()
    blockchain.writeBlockchain()
    print("bloque añadido...")
